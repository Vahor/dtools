use anyhow::Result;
use tauri::AppHandle;

pub mod constants;
pub mod downloader;

#[derive(Debug)]
pub struct DataExtractor {
    pub downloader: downloader::Downloader,
}

impl DataExtractor {
    pub fn new(handler: &AppHandle) -> Result<DataExtractor> {
        let instance = DataExtractor {
            downloader: downloader::Downloader::new(handler)?,
        };

        return Ok(instance);
    }
}
