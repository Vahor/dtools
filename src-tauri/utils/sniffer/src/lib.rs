use anyhow::Result;
use tauri::AppHandle;

pub mod network;
pub mod parser;
pub mod protocol;

#[derive(Debug)]
pub struct PacketReader {
    pub network: network::PacketListener,
    pub protocol: protocol::ProtocolManager,
}

impl PacketReader {
    pub fn new(handle: &AppHandle) -> Result<PacketReader> {
        let instance = PacketReader {
            protocol: protocol::ProtocolManager::new(handle)?,
            network: network::PacketListener::new(),
        };

        return Ok(instance);
    }
}
