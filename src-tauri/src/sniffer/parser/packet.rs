use serde_json::{Map, Number, Value};
use tauri::{App, AppHandle};
use tracing::info;

use crate::sniffer::protocol::ProtocolManager;

use super::{metadata::PacketMetadata, wrapper::DataWrapper};

#[derive(Debug, Clone)]
pub struct Packet {
    pub id: u16,
    pub data: Map<String, Value>,
}

impl Packet {}

#[derive(Debug, Clone)]
pub struct PacketParser {
    pub id: u16,
    pub data: DataWrapper,
}

impl PacketParser {
    pub fn new(id: u16, data: DataWrapper) -> Self {
        PacketParser { id, data }
    }

    pub fn from_metadata(meta: &PacketMetadata) -> Self {
        PacketParser::new(meta.id, DataWrapper::new(meta.data.clone()))
    }

    pub fn parse(&mut self, protocol_manager: &ProtocolManager) -> Option<Packet> {
        match protocol_manager.get_event(&self.id) {
            Some(event) => {
                let mut packet = Packet {
                    id: self.id,
                    data: Map::new(),
                };

                Some(packet)
            }
            None => {
                info!("Unknown packet: {}", self.id);
                None
            }
        }
    }
}
