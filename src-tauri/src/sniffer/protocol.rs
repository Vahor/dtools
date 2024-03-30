use std::{collections::HashMap, path::Path};
use thiserror::Error;

use serde::{Deserialize, Serialize};
use serde_aux::field_attributes::deserialize_option_number_from_string;
use tracing::info;

use crate::constants::{EVENTS_FILE, EXTRACTOR_DIR};

pub type FieldName = String;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum EventName {
    #[serde(other)]
    Unknown,
}

pub type EventId = u16;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
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
    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProtocolEvent {
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub id: Option<EventId>,
    #[serde(rename = "class_name")]
    pub name: EventName,
    #[serde(rename = "superclass")]
    pub parent: Option<EventName>,
    pub attributes: HashMap<FieldName, ProtocolVarType>,
}

#[derive(Debug)]
pub struct ProtocolManager {
    event_by_id: HashMap<EventId, ProtocolEvent>,
    event_by_class: HashMap<EventName, EventId>,
}

fn load_protocol(
    protocol_file_path: impl AsRef<Path>,
) -> Result<HashMap<EventId, ProtocolEvent>, std::io::Error> {
    let protocol_file_path = protocol_file_path.as_ref();
    let protocol_file_path = protocol_file_path.join(EXTRACTOR_DIR).join(EVENTS_FILE);

    let mut event_by_id = HashMap::new();

    assert!(
        protocol_file_path.exists(),
        "Protocol file not found at {}",
        protocol_file_path.display()
    );

    let content = std::fs::read_to_string(&protocol_file_path)?;
    let protocol: Vec<ProtocolEvent> = serde_json::from_str(&content)?;

    for event in protocol {
        if let Some(id) = event.id {
            event_by_id.insert(id, event);
        }
    }
    return Ok(event_by_id);
}

impl ProtocolManager {
    pub fn new(protocol_file_path: impl AsRef<Path>) -> Result<ProtocolManager, ProtocolError> {
        let event_by_id = load_protocol(protocol_file_path)?;
        let event_by_class: HashMap<EventName, EventId> =
            event_by_id
                .iter()
                .fold(HashMap::new(), |mut map, (id, event)| {
                    map.insert(event.name.clone(), *id);
                    return map;
                });

        let instance = ProtocolManager {
            event_by_id,
            event_by_class,
        };

        info!("Loaded {} events", instance.event_by_id.len());
        return Ok(instance);
    }

    pub fn get_event(&self, id: &EventId) -> Option<&ProtocolEvent> {
        self.event_by_id.get(id)
    }

    pub fn get_event_by_class(&self, class: &EventName) -> Option<&ProtocolEvent> {
        let id = self.event_by_class.get(class)?;
        return self.get_event(id);
    }
}

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
