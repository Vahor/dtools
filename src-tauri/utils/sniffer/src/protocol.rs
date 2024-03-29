use std::collections::HashMap;

use anyhow::Result;
use extractor::constants::EVENTS_FILE;
use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_option_number_from_string;
use tauri::{path::BaseDirectory, AppHandle, Manager};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ProtocolVarType {
    UTF,
    VarUhShort,
    VarShort,
    Short,
    Float,
    VarUhLong,
    VarLong,
    Byte,
    VarUhInt,
    Int,
    Double,
    Boolean,
    UnsignedInt,
    UnsignedShort,
    VarInt,
    UnsignedByte,
    ByteArray,
    False,

    #[serde(other)]
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProtocolEvent {
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    id: Option<u16>,
    class_name: String,
    superclass: Option<String>,
    attributes: HashMap<String, ProtocolVarType>,
}

#[derive(Debug)]
pub struct ProtocolManager {}

impl ProtocolManager {
    pub fn new(handler: &AppHandle) -> Result<ProtocolManager> {
        let mut instance = ProtocolManager {};
        instance.load_protocol(handler)?;

        return Ok(instance);
    }

    fn load_protocol(&mut self, handler: &AppHandle) -> Result<()> {
        let protocol_file = handler
            .path()
            .resolve(EVENTS_FILE.clone(), BaseDirectory::AppData)?;

        assert!(protocol_file.exists(), "Protocol file not found");

        let content = std::fs::read_to_string(&protocol_file)?;
        let protocol: Vec<ProtocolEvent> = serde_json::from_str(&content)?;

        dbg!(&protocol);
        return Ok(());
    }
}
