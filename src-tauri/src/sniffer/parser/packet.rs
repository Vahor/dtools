use std::collections::HashMap;

use indexmap::IndexMap;
use serde_json::{Map, Number, Value};
use thiserror::Error;
use tracing::{debug, info, warn};

use crate::sniffer::protocol::{
    EventId, EventName, ProtocolManager, ProtocolSchema, ProtocolVarType,
};

use super::{metadata::PacketMetadata, wrapper::DataWrapper};

type PacketData = Map<String, Value>;

#[derive(Debug, Clone)]
pub struct Packet {
    pub id: u16,
    pub name: EventName,
    pub data: PacketData,
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

    pub fn parse(&mut self, protocol_manager: &ProtocolManager) -> Result<Packet, PacketError> {
        match protocol_manager.get_protocol(&self.id) {
            Some(event) => {
                debug!("Parsing packet: {}", event.name);
                let data = self.parse_packet_data(protocol_manager, event)?;

                Ok(Packet {
                    id: self.id,
                    name: event.name.clone(),
                    data,
                })
            }
            None => Err(PacketError::UnknownPacketType),
        }
    }

    fn parse_packet_data(
        &mut self,
        protocol_manager: &ProtocolManager,
        event: &ProtocolSchema,
    ) -> Result<PacketData, PacketError> {
        let mut data = Map::new();

        if let Some(parent) = &event.parent {
            let parent_type = protocol_manager.get_protocol_by_class(&parent);
            if parent_type.is_none() {
                debug!("Unknown parent type: {:?}", parent);
                return Err(PacketError::UnknownParentType(parent.clone()));
            }
            let parent_type = parent_type.unwrap();
            let parent_data = self.parse_packet_data(protocol_manager, parent_type)?;
            data.extend(parent_data);
        }
        data.extend(self.parse_packet_attributes(&event.attributes)?);

        Ok(data)
    }

    fn parse_packet_attributes(
        &mut self,
        attributes: &IndexMap<String, ProtocolVarType>,
    ) -> Result<PacketData, PacketError> {
        let mut data = Map::new();

        for (name, var_type) in attributes {
            let value = self.parse_attribute(var_type);
            debug!("Remaining data: {:?}", self.data.get_remaining());
            data.insert(name.clone(), value);
        }

        Ok(data)
    }

    fn parse_attribute(&mut self, var_type: &ProtocolVarType) -> Value {
        warn!("Remaining data: {:?}", self.data.get_remaining());
        debug!("Parsing attribute: {:?}", var_type);
        match var_type {
            ProtocolVarType::Byte => {
                let value = self.data.read_byte();
                Value::Number(Number::from(value))
            }
            ProtocolVarType::String => {
                let value = self.data.read_utf();
                Value::String(value)
            }
            ProtocolVarType::VarInt => {
                let value = self.data.read_var_int();
                Value::Number(Number::from(value))
            }
            ProtocolVarType::VarShort => {
                let value = self.data.read_var_short();
                Value::Number(Number::from(value))
            }
            ProtocolVarType::VarLong => {
                let value = self.data.read_var_long();
                Value::Number(Number::from(value))
            }
            ProtocolVarType::Int => {
                let value = self.data.read_int();
                Value::Number(Number::from(value))
            }
            ProtocolVarType::Short => {
                let value = self.data.read_short();
                Value::Number(Number::from(value))
            }
            _ => {
                warn!("Unknown attribute type: {:?}", var_type);
                Value::Null
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Unknown parent type")]
    UnknownParentType(EventName),
    #[error("Unknown packet type")]
    UnknownPacketType,
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_parse_packet() {
        let chat_server_message = "9c760e4f1efc8c97ea31a27608004500007ef5ac40003906ea12ac41f3a5c0a8012b15b3c9c6ed170b90c52a70c180180008a01100000101080a3988ebc7b293068014614709001b68747470733a2f2f646973636f72642e67672f4678705670324e63660731df0008727671796f3131674261732fa024c000000b546574616c6f726164726100000aa5b9c2";
        let house_properties_message = "9c760e4f1efc8c97ea31a2760800450002bcf5aa40003906e7d6ac41f3a5c0a8012b15b3c9c6ed1708efc52a70c180180008237e00000101080a3988eb76b293064f855921c6941c00010007db0d0f660600000008000848656c6c65626f7200043232333600855951c7941c00010007db0e118406000000010013756e70736575646f766f75736176657a64697400043538363700c51200154c6120636f6d7061676e69652064657320466f75782dc70300f17901040007030085592cc7941c00010007db0e0f66060000000200134d696e6b612d4d616972652d6441737472756200043832393700855924c7941c00010007db0e0f661600000003000b2d2d4d6f6f6e73686f742d00043434313900855920c7941c00010007db0e0f66040000000400074d696e6b612d3900043137383100855939c7941c00010007db0e11841600000006000945636174696e686f6f00043135383100b75900074e6f2d48616b694dac0300ba00041500859ef1855954c7941c00010007db0e118404000000080012436c616e2d6465732d53656c656e7974657300043536323900a34100194c65732041707072656e746973204176656e74757269657273368f0100edc61a1c004d1c1d855923c7941c00010007db0e0f661600000009000a70657469736f6c65696d00043431353800855925c7941c00010007db0e0f66060000000a000c6775697a696d616d6f756e650004373232350021c904ffffffff4d6d120003014601a501a400014233d2d30126000014619709006b486579205661686f722c20747520766575782061636865746572206f75206dc3aa6d652076656e64726520746573206b616d6173206175206d65696c6c65757220707269782073616e73207269737175652063276573742069636920717565206361207365207061737365660731df0008333773727973327a4261732fa024c000000b546574616c6f726164726100000aa5b9c2";
        let hex_streams = [chat_server_message, house_properties_message];
        let path = "tests/fixtures/".to_string();
        let path = Path::new(&path);
        let procol_manager = ProtocolManager::new(path).unwrap();

        for hex_stream in hex_streams.iter() {
            let hex = (0..hex_stream.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex_stream[i..i + 2], 16).unwrap())
                .collect::<Vec<u8>>();

            let current_frame_buffer = &mut DataWrapper::new(hex);
            let metadata = PacketMetadata::from_buffer(current_frame_buffer).unwrap();
            let mut parser = PacketParser::from_metadata(&metadata);

            let packet = parser.parse(&procol_manager).unwrap();
        }
    }
}
