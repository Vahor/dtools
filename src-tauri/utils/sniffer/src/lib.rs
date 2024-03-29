use anyhow::Result;
use tauri::AppHandle;

pub mod protocol;

#[derive(Debug)]
pub struct PacketReader {
    pub whitelist: Vec<String>,
    protocol: protocol::ProtocolManager,
}

impl PacketReader {
    pub fn new(handler: &AppHandle) -> Result<PacketReader> {
        let instance = PacketReader {
            whitelist: vec![],
            protocol: protocol::ProtocolManager::new(handler)?,
        };

        return Ok(instance);
    }

    pub fn sniff(&self, url: &str) -> bool {
        println!("Sniffing: {}", url);
        true
    }
}
