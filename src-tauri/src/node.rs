use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex, RwLock},
};

use crate::{
    config::{self, NodeConfig},
    downloader,
};
use crate::{
    features,
    sniffer::{network, protocol},
};
use thiserror::Error;
use tracing::{error, info};
use tracing_appender::{
    non_blocking::{NonBlocking, WorkerGuard},
    rolling::{RollingFileAppender, Rotation},
};
use tracing_subscriber::{filter::FromEnvError, prelude::*, EnvFilter};

#[derive(Debug)]
pub struct Node {
    pub data_dir: PathBuf,
    pub config: Arc<config::Manager<NodeConfig>>,
    pub http: reqwest::Client,
    pub downloader: Arc<Mutex<downloader::Downloader>>,
    pub packet_listener: Arc<Mutex<network::PacketListener>>,
    pub protocol: Arc<protocol::protocol::ProtocolManager>,

    pub handle: Option<tauri::AppHandle>,

    pub features: Features,

    /// Temporary store for data, often use in the packet listener
    pub store: Arc<Mutex<HashMap<String, String>>>,
}

#[derive(Debug)]
pub struct Features {
    pub chat: Arc<RwLock<features::chat::feature::ChatFeature>>,
}

impl Node {
    pub async fn new(
        data_dir: impl AsRef<Path>,
        handle: Option<tauri::AppHandle>,
        init: bool,
    ) -> Result<Arc<Node>, NodeError> {
        let data_dir_path = data_dir.as_ref();

        let _ = fs::create_dir_all(data_dir_path)?;

        let _ = Self::init_logger(data_dir_path)?;
        info!("Data directory: {}", data_dir_path.display());

        let config = config::Manager::new(data_dir_path, "config.json")
            .await
            .map_err(NodeError::FailedToInitializeConfig)?;

        let http_client = reqwest::Client::new();

        let protocol = protocol::protocol::ProtocolManager::new(data_dir_path)
            .map_err(NodeError::FailedToInitializeProtocol)?;

        let packet_listener = network::PacketListener::new();
        let downloader = downloader::Downloader::new();

        let features = Features {
            chat: Arc::new(RwLock::new(features::chat::feature::ChatFeature::new())),
        };

        let node = Arc::new(Node {
            data_dir: data_dir_path.to_path_buf(),
            config,
            downloader: Arc::new(Mutex::new(downloader)),
            http: http_client,
            protocol: Arc::new(protocol),
            packet_listener: Arc::new(Mutex::new(packet_listener)),
            handle,
            features,
            store: Arc::new(Mutex::new(HashMap::new())),
        });

        node.packet_listener.lock().unwrap().set_node(node.clone());
        node.features
            .chat
            .write()
            .unwrap()
            .set_node(node.clone())
            .await;

        if init {
            node.downloader.lock().unwrap().init(&node).await?;
            node.packet_listener.lock().unwrap().run()?;
        }
        info!("Node initialized successfully");

        return Ok(node);
    }

    pub fn init_logger(data_dir: &Path) -> Result<WorkerGuard, FromEnvError> {
        let log_dir = data_dir.join("logs");
        let (log_file, guard) = NonBlocking::new(
            RollingFileAppender::builder()
                .filename_prefix("node.log")
                .max_log_files(7)
                .rotation(Rotation::DAILY)
                .build(log_dir)
                .expect("Failed to create log file"),
        );

        // set RUST_LOG env var to enable logging
        if std::env::var("RUST_LOG").is_err() {
            std::env::set_var("RUST_LOG", "debug");
        }

        tracing_subscriber::registry()
            .with(
                tracing_subscriber::fmt::layer()
                    .with_file(true)
                    .with_line_number(true)
                    .with_writer(log_file)
                    .with_ansi(false)
                    .with_filter(EnvFilter::from_default_env()),
            )
            .with(
                tracing_subscriber::fmt::layer()
                    .with_file(true)
                    .with_line_number(true)
                    .with_writer(std::io::stdout)
                    .with_filter(EnvFilter::from_default_env()),
            )
            .init();

        std::panic::set_hook(Box::new(move |panic| {
            if let Some(location) = panic.location() {
                tracing::error!(
                    message = %panic,
                    panic.file = format!("{}:{}", location.file(), location.line()),
                    panic.column = location.column(),
                );
            } else {
                tracing::error!(message = %panic);
            }
        }));

        return Ok(guard);
    }
}

#[derive(Debug, Error)]
pub enum NodeError {
    #[error("Failed to initialize ConfigManager")]
    FailedToInitializeConfig(#[from] config::ConfigError),
    #[error("Failed to initialize Downloader")]
    FailedToInitializeDownloader(#[from] downloader::DownloaderError),
    #[error("Failed to initialize logger")]
    FailedToInitializeLogger(#[from] FromEnvError),
    #[error("Failed to create data directory")]
    FailedToCreateDataDir(#[from] std::io::Error),
    #[error("Failed to initialize ProtocolManager")]
    FailedToInitializeProtocol(#[from] protocol::protocol::ProtocolError),
    #[error("Failed to run packet listener")]
    FailedToRunPacketListener(#[from] network::PacketListenerError),
}
