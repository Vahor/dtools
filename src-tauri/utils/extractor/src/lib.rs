use anyhow::Result;
use tauri::AppHandle;

pub mod constants;
pub mod downloader;

#[derive(Debug)]
pub struct DataExtractor {
    pub downloader: downloader::Downloader,
}

impl DataExtractor {
    pub fn new(handle: &AppHandle) -> Result<DataExtractor> {
        let instance = DataExtractor {
            downloader: downloader::Downloader::new(handle)?,
        };

        return Ok(instance);
    }
}
