use serde::{Deserialize, Serialize};
use std::{
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};
use thiserror::Error;
use tracing::info;

use crate::{
    constants::CONFIG_FILE_NAME, features::chat::config::ChatConfig, sniffer::config::NetworkConfig,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeConfig {
    pub game_version: String,
    pub network: NetworkConfig,
    pub features: FeaturesConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeaturesConfig {
    pub chat: ChatConfig,
}

impl NodeConfig {
    pub async fn load(path: impl AsRef<Path>) -> Result<NodeConfig, NodeConfigError> {
        let path = path.as_ref();
        let content = std::fs::read_to_string(path)?;
        let config: NodeConfig = serde_json::from_str(&content)?;
        Ok(config)
    }

    pub async fn save(&self, path: impl AsRef<Path>) -> Result<(), NodeConfigError> {
        let config = serde_json::to_string_pretty(self)?;
        std::fs::write(path, config)?;
        Ok(())
    }

    pub fn save_sync(&self, path: impl AsRef<Path>) -> Result<(), NodeConfigError> {
        let config = serde_json::to_string_pretty(self)?;
        std::fs::write(path, config)?;
        Ok(())
    }
}

impl Default for NodeConfig {
    fn default() -> Self {
        NodeConfig {
            game_version: "".to_string(),
            network: NetworkConfig::default(),
            features: FeaturesConfig {
                chat: ChatConfig::default(),
            },
        }
    }
}

#[derive(Debug)]
pub struct Manager {
    pub config: RwLock<NodeConfig>,
    pub data_dir_path: PathBuf,
    pub config_file_path: PathBuf,
}

impl Manager {
    pub async fn new(data_dir: impl AsRef<Path>) -> Result<Arc<Self>, NodeConfigError> {
        let data_dir = data_dir.as_ref();
        let data_dir_path = data_dir.to_path_buf();
        let config_file_path = data_dir.join(CONFIG_FILE_NAME);

        let config = if config_file_path.exists() {
            NodeConfig::load(&config_file_path).await?
        } else {
            let default = NodeConfig::default();
            default.save(&config_file_path).await?;
            default
        };

        let config = RwLock::new(config);

        info!("Config loaded successfully from {:?}", config_file_path);
        Ok(Arc::new(Manager {
            config,
            data_dir_path,
            config_file_path,
        }))
    }

    pub async fn update_config(
        &self,
        update_fn: impl FnOnce(&mut NodeConfig),
    ) -> Result<(), NodeConfigError> {
        let mut config = self.config.write().unwrap();
        update_fn(&mut config);
        config
            .save(&self.config_file_path)
            .await
            .map_err(Into::into)
    }

    pub fn update_config_sync(
        &self,
        update_fn: impl FnOnce(&mut NodeConfig),
    ) -> Result<(), NodeConfigError> {
        let mut config = self.config.write().unwrap();
        update_fn(&mut config);
        config.save_sync(&self.config_file_path).map_err(Into::into)
    }
}

#[derive(Error, Debug)]
pub enum NodeConfigError {
    #[error(transparent)]
    FileIo(#[from] std::io::Error),
    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}
