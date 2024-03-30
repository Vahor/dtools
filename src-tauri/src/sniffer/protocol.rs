use indexmap::IndexMap;
use std::{collections::HashMap, path::Path};
use thiserror::Error;

use serde::*;
use serde_aux::field_attributes::deserialize_option_number_from_string;
use serde_enum_str::Deserialize_enum_str;
use tracing::info;

use crate::constants::{EVENTS_FILE, EXTRACTOR_DIR};

pub type FieldName = String;
pub type EventName = String;

pub type EventId = u16;

#[derive(Deserialize_enum_str, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProtocolVarType {
    String,
    VarInt,
    VarLong,
    VarShort,
    Short,
    Int,
    Byte,

    #[serde(other)]
    Unknown(String),
}

impl ProtocolVarType {
    pub fn is_primitive(&self) -> bool {
        match self {
            ProtocolVarType::String
            | ProtocolVarType::VarInt
            | ProtocolVarType::VarLong
            | ProtocolVarType::VarShort
            | ProtocolVarType::Short
            | ProtocolVarType::Int
            | ProtocolVarType::Byte => true,
            _ => false,
        }
    }

    pub fn is_unknown(&self) -> bool {
        match self {
            ProtocolVarType::Unknown(_) => true,
            _ => false,
        }
    }
}

#[derive(Deserialize, Debug)]
pub struct ProtocolSchema {
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub id: Option<EventId>,
    #[serde(rename = "class_name")]
    pub name: EventName,
    #[serde(rename = "superclass")]
    pub parent: Option<EventName>,
    pub attributes: IndexMap<FieldName, ProtocolVarType>,
}

#[derive(Debug)]
pub struct ProtocolManager {
    protocol_by_id: HashMap<EventId, ProtocolSchema>,
    protocol_id_by_name: HashMap<EventName, EventId>,
}

fn load_protocol(
    protocol_file_path: impl AsRef<Path>,
) -> Result<HashMap<EventId, ProtocolSchema>, std::io::Error> {
    let protocol_file_path = protocol_file_path.as_ref();
    let protocol_file_path = protocol_file_path.join(EXTRACTOR_DIR).join(EVENTS_FILE);

    let mut event_by_id = HashMap::new();

    assert!(
        protocol_file_path.exists(),
        "Protocol file not found at {}",
        protocol_file_path.display()
    );

    let content = std::fs::read_to_string(&protocol_file_path)?;
    let protocol: Vec<ProtocolSchema> = serde_json::from_str(&content)?;

    for event in protocol {
        if let Some(id) = event.id {
            event_by_id.insert(id, event);
        }
    }
    return Ok(event_by_id);
}

impl ProtocolManager {
    pub fn new(protocol_file_path: impl AsRef<Path>) -> Result<ProtocolManager, ProtocolError> {
        let protocol_by_id = load_protocol(protocol_file_path)?;
        let protocol_id_by_name: HashMap<EventName, EventId> =
            protocol_by_id
                .iter()
                .fold(HashMap::new(), |mut map, (id, event)| {
                    map.insert(event.name.clone(), *id);
                    return map;
                });

        let instance = ProtocolManager {
            protocol_by_id,
            protocol_id_by_name,
        };

        info!("Loaded {} protocols", instance.protocol_by_id.len());
        return Ok(instance);
    }

    pub fn get_protocol(&self, id: &EventId) -> Option<&ProtocolSchema> {
        self.protocol_by_id.get(id)
    }

    pub fn get_protocol_by_class(&self, class: &EventName) -> Option<&ProtocolSchema> {
        if let Some(id) = self.protocol_id_by_name.get(class) {
            return self.get_protocol(id);
        }
        None
    }
}

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
