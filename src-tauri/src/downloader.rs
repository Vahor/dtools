use crate::constants::{DATA_URL, EXTRACTOR_DIR, VERSION_REGEX};
use crate::node::Node;
use fs_extra::dir::move_dir;
use fs_extra::dir::remove;
use std::path::PathBuf;
use tempdir::TempDir;
use thiserror::Error;
use tracing::{debug, info};

#[derive(Debug)]
pub struct Downloader {
    pub latest_version: Option<String>,
}

impl Downloader {
    pub fn new() -> Downloader {
        return Downloader {
            latest_version: None,
        };
    }

    pub async fn init(&mut self, node: &Node) -> Result<(), DownloaderError> {
        self.latest_version = self.get_latest_version(&node.http).await;

        let res = match self.latest_version {
            Some(ref version) => {
                let need_update = {
                    let current_version = &node.config.config.read().unwrap().game_version;
                    info!("Current version: {}", &current_version);
                    current_version != version
                };
                info!("Latest version: {}", &version);

                if need_update {
                    let version = version.to_string();
                    let res = self
                        .download(&node.http, &version, node.data_dir.clone())
                        .await;

                    debug!("Updating config with new version");

                    node.config
                        .update_config(|config| {
                            config.game_version = version;
                        })
                        .await?;
                    debug!("Config updated successfully");
                    res
                } else {
                    Ok(())
                }
            }
            None => Err(DownloaderError::FailedToDownloadLatestVersion),
        };

        return res;
    }

    pub async fn get_latest_version(&mut self, client: &reqwest::Client) -> Option<String> {
        let latest_data_url = format!("{}/latest", DATA_URL);
        let response = client.head(&latest_data_url).send().await.unwrap();
        let url = response.url().as_str();

        let final_segment = url.split('/').last().unwrap();
        assert!(
            VERSION_REGEX.is_match(final_segment),
            "Invalid version format"
        );

        return Some(final_segment.to_string());
    }

    pub async fn download(
        &mut self,
        client: &reqwest::Client,
        version: &String,
        data_dir: PathBuf,
    ) -> Result<(), DownloaderError> {
        info!("Downloading version: {}", version);
        let download_url = format!("{}/download/{}/data.zip", DATA_URL, version);
        let temp_dir = TempDir::new("downloader").expect("Failed to create temp dir");

        let response = client.get(&download_url).send().await?;
        let data = response.bytes().await?;

        // Download data.zip
        let zip_file_path = temp_dir.path().join("data.zip");
        std::fs::write(&zip_file_path, data)
            .expect(format!("Failed to write data.zip to {:?}", zip_file_path).as_str());
        info!("Downloaded data.zip");

        // Unzip data.zip
        let tmp_unzip_dir = temp_dir.path().join("unzip");
        let dist_folder = data_dir.join(EXTRACTOR_DIR);
        // cleanup dist folder
        remove(&dist_folder).expect(format!("Failed to remove {:?}", dist_folder).as_str());

        // create parent directory if not exists
        if !dist_folder.exists() {
            std::fs::create_dir_all(&dist_folder)?;
        }

        let zip_file = std::fs::File::open(&zip_file_path)
            .expect(format!("Failed to open {:?}", zip_file_path).as_str());
        let mut archive = zip::ZipArchive::new(zip_file).expect("Failed to read ZipArchive");
        archive
            .extract(&tmp_unzip_dir)
            .expect(format!("Failed to extract data.zip to {:?}", tmp_unzip_dir).as_str());
        remove(&tmp_unzip_dir.join("data/A/DofusInvoker")).expect("Failed to remove DofusInvoker");

        let options = fs_extra::dir::CopyOptions::new().content_only(true);
        let from = vec![
            tmp_unzip_dir.join("data/A"),
            tmp_unzip_dir.join("data/B"),
            tmp_unzip_dir.join("data/C"),
        ];
        for f in from {
            move_dir(&f, &dist_folder, &options).expect("Failed to move items");
        }
        info!("Download completed successfully");

        temp_dir.close()?;

        return Ok(());
    }
}

#[derive(Error, Debug)]
pub enum DownloaderError {
    #[error("Failed to download latest version")]
    FailedToDownloadLatestVersion,
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ConfigError(#[from] crate::config::NodeConfigError),
}
