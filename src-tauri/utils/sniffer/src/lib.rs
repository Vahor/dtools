use anyhow::Result;
use downloader::Downloader;
use tauri::AppHandle;

pub mod downloader;
pub mod protocol;

#[derive(Debug)]
pub struct PacketReader {
    pub whitelist: Vec<String>,
    pub downloader: Downloader,
}

impl PacketReader {
    pub fn new(handler: &AppHandle) -> Result<PacketReader> {
        let instance = PacketReader {
            downloader: Downloader::new(handler)?,
            whitelist: vec![],
        };

        return Ok(instance);
    }

    pub fn sniff(&self, url: &str) -> bool {
        println!("Sniffing: {}", url);
        true
    }
}
