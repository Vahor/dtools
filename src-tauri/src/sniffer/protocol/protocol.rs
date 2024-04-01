use indexmap::IndexMap;
use std::{
    collections::HashMap,
    fmt::{Debug, Display},
    path::Path,
};
use thiserror::Error;

use serde::*;
use serde_aux::field_attributes::deserialize_option_number_from_string;
use tracing::info;

use crate::constants::{EVENTS_FILE, EXTRACTOR_DIR};

pub type FieldName = String;
pub type EventName = String;

pub type EventId = u16;

#[derive(Debug)]
pub enum KnownEvent {
    ChatServerMessage,
    ChatServerWithObjectMessage,
}

impl Display for KnownEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return Debug::fmt(self, f);
    }
}

#[derive(Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ProtocolVarType {
    String,
    VarInt,
    VarLong,
    VarShort,
    Short,
    Int,
    Byte,
    None,
    Boolean,
    Double,

    #[serde(untagged)]
    Other(EventName),
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
            | ProtocolVarType::Boolean
            | ProtocolVarType::None
            | ProtocolVarType::Double
            | ProtocolVarType::Byte => true,
            _ => false,
        }
    }

    // The goal is to decompose Vector<ProcolVarType, ProtocolVarType> into a single ProtocolVarType
    // same for TypeId<ProtocolVarType> and HashMap<ProtocolVarType, ProtocolVarType>
    //
    pub fn is_complex(&self) -> bool {
        !self.is_primitive()
    }

    pub fn parse_vector(&self) -> Option<ProtocolVarTypeVector> {
        match self {
            ProtocolVarType::Other(name) => {
                let is_vector = name.starts_with("Vector<"); // TODO: clean this up, it's ugly
                let is_type_id_vector = name.starts_with("TypeIdVector<");
                if is_vector || is_type_id_vector {
                    let name = match is_vector {
                        true => name.trim_start_matches("Vector<").trim_end_matches(">"),
                        false => name
                            .trim_start_matches("TypeIdVector<")
                            .trim_end_matches(">"),
                    };
                    let mut parts = name.split(",");
                    let a = parts.next().unwrap().trim();
                    let b = parts.next().unwrap().trim();

                    let a = serde_plain::from_str::<ProtocolVarType>(a).unwrap();
                    let b = serde_plain::from_str::<ProtocolVarType>(b).unwrap();

                    return Some(ProtocolVarTypeVector {
                        length: a,
                        types: b,
                    });
                }
                None
            }
            _ => None,
        }
    }

    pub fn parse_type_id(&self) -> Option<ProtocolVarType> {
        match self {
            ProtocolVarType::Other(name) => {
                if name.starts_with("TypeId<") {
                    let name = name.trim_start_matches("TypeId<").trim_end_matches(">");
                    let a = serde_plain::from_str::<ProtocolVarType>(name).unwrap();
                    return Some(a);
                }
                None
            }
            _ => None,
        }
    }
}

pub struct ProtocolVarTypeVector {
    pub length: ProtocolVarType,
    pub types: ProtocolVarType,
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

    let mut without_id_count = 0;
    for event in protocol {
        if let Some(id) = event.id {
            event_by_id.insert(id, event);
        } else {
            // There should be only one event without an id (NetworkMessage)
            assert_eq!(without_id_count, 0);
            without_id_count += 1;
            event_by_id.insert(0, event);
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

    pub fn get_protocol_id_by_class(&self, class: &EventName) -> Option<&EventId> {
        self.protocol_id_by_name.get(class)
    }
}

#[derive(Error, Debug)]
pub enum ProtocolError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}
