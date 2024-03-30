use std::path::PathBuf;

use anyhow::Result;

pub mod network;
pub mod parser;
pub mod protocol;

#[derive(Debug)]
pub struct PacketReader {
    pub network: network::PacketListener,
    pub protocol: protocol::ProtocolManager,
}

impl PacketReader {
    pub fn new(protocol_file_path: PathBuf) -> Result<PacketReader> {
        let instance = PacketReader {
            protocol: protocol::ProtocolManager::new(protocol_file_path)?,
            network: network::PacketListener::new(),
        };

        return Ok(instance);
    }
}
