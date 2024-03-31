use indexmap::IndexMap;
use serde_json::{Map, Number, Value};
use thiserror::Error;
use tracing::{debug, info};

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
                let data = self.parse_packet_data(protocol_manager, event)?;

                // check if there is any data left
                if !self.data.get_remaining().is_empty() {
                    debug!("Data left after parsing: {:?}", self.data.get_remaining());
                }

                Ok(Packet {
                    id: self.id,
                    name: event.name.clone(),
                    data,
                })
            }
            None => Err(PacketError::UnknownPacketType(self.id)),
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
                return Err(PacketError::UnknownParentType(parent.clone()));
            }

            let parent_type = parent_type.unwrap();
            if parent_type.id.is_some() {
                let parent_data = self.parse_packet_data(protocol_manager, parent_type)?;
                data.extend(parent_data);
            }
        }
        data.extend(self.parse_packet_attributes(protocol_manager, &event.attributes)?);

        Ok(data)
    }

    fn parse_packet_attributes(
        &mut self,
        protocol_manager: &ProtocolManager,
        attributes: &IndexMap<String, ProtocolVarType>,
    ) -> Result<PacketData, PacketError> {
        let mut data = Map::new();

        for (name, var_type) in attributes {
            // info!("Parsing attribute: {} {:?}", name, var_type);
            let value = self.parse_attribute(protocol_manager, var_type)?;
            // debug!("Parsed attribute: {} {:?} => {:?}", name, var_type, value);
            data.insert(name.clone(), value);
        }

        Ok(data)
    }

    fn parse_attribute(
        &mut self,
        protocol_manager: &ProtocolManager,
        var_type: &ProtocolVarType,
    ) -> Result<Value, PacketError> {
        std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let res = match var_type {
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
                    let value = self.data.read_unsigned_short();
                    Value::Number(Number::from(value))
                }
                ProtocolVarType::None => Value::Null,
                ProtocolVarType::Boolean => {
                    let value = self.data.read_byte() == 1;
                    Value::Bool(value)
                }
                ProtocolVarType::Double => {
                    let value = self.data.read_double();
                    Value::Number(Number::from_f64(value as f64).unwrap())
                }
                ProtocolVarType::Other(name) => {
                    return self.parse_complexe_type(protocol_manager, name, var_type);
                }
            };
            Ok(res)
        }))
        .unwrap_or_else(|_| Err(PacketError::FailedToParseAttribute(var_type.clone())))
    }

    pub fn parse_complexe_type(
        &mut self,
        protocol_manager: &ProtocolManager,
        name: &String,
        var_type: &ProtocolVarType,
    ) -> Result<Value, PacketError> {
        if let Some(vector) = var_type.parse_vector() {
            match vector.length {
                ProtocolVarType::Short => {
                    let length = self.data.read_unsigned_short(); // array length is signed
                    let mut values = Vec::with_capacity(length as usize);
                    for _ in 0..length {
                        let value = self.parse_attribute(protocol_manager, &vector.types)?;
                        values.push(value);
                    }
                    return Ok(Value::Array(values));
                }
                _ => {}
            }
            return Err(PacketError::FailedToParseAttribute(var_type.clone()));
        } else if let Some(type_id) = var_type.parse_type_id() {
            return self.parse_attribute(protocol_manager, &type_id);
        } else {
            let schema = protocol_manager.get_protocol_by_class(name);
            if schema.is_none() {
                return Err(PacketError::UnknownParentType(name.clone()));
            }
            let schema = schema.unwrap();
            let value = self.parse_packet_data(protocol_manager, schema)?;
            Ok(Value::Object(value))
        }
    }
}

#[derive(Debug, Error)]
pub enum PacketError {
    #[error("Unknown parent type")]
    UnknownParentType(EventName),
    #[error("Unknown packet type")]
    UnknownPacketType(EventId),
    #[error("Failed to parse attribute")]
    FailedToParseAttribute(ProtocolVarType),
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::node::Node;

    use super::*;
    use crate::sniffer::parser::metadata::PacketHeader;
    use tracing::info;

    #[test]
    fn test_parse_packet() {
        let chat_server_message_commerce = "9c760e4f1efc8c97ea31a276080045000078ca9740003906152eac41f3a5c0a8012b15b3c5b016b74d1cdf5e09cf801800083eaa00000101080a96b2b20290e8d54514614105000e56656e6420737475666620616972660953cd0008387a6d71616d32654231d39501260000001253616e637475732d42656e65646963747573000002e0ba5b";
        let char_server_message_commerce_items = "9c760e4f1efc8c97ea31a2760800450000b8caeb40003906149aac41f3a5c0a8012b15b3c5b016b75f74df5e0ac280180008b15600000101080a96b830f290ee56d920498105002756454e4420efbfbc20efbfbc206574204143484554544520efbfbc20394d3520efbfbc20354d35660955350008673732757733357642610aa42024c000000f43616e61696c6c652d656e2d746f630000018f4ef500040000fcd20100000000000000fed20100000000000000e8a30100000000000000e1a3010000000100";
        let chat_server_message_recruitment = "9c760e4f1efc8c97ea31a27608004500008aca9840003906151bac41f3a5c0a8012b15b3c5b016b74d60df5e09cf80180008c5ab00000101080a96b2b65e90e8dd1014615306002c48656c6c6f207175656c717527756e20706f7572206d6520666169726520646a206b77616b7761203f207829660953ce00086c33376633353172426173fb2024c00000065175696b7965000005974ec9";
        let chat_server_message_recruitment_map = "9c760e4f1efc8c97ea31a2760800450000c4ca954000390614e4ac41f3a5c0a8012b15b3c5b016b74c3ddf5e09cf80180008b59b00000101080a96b294e290e88ef314618d060065426f6f6e6a6f7572206a65206368657263686520756e20736164692066656d656c6c6520706f7572207365206d65747472652073757220756e652064616c6c6520737670206c657320616d696573206a652070617965207b6d61702c2d31322c2d362c317d660953c50008626432366275396342465c008093000000075065682d546168000008c4ed89";
        let map_complementary_informations_data_message = "9c760e4f1efc8c97ea31a27608004500055df4ab40003906e634ac41f3a5c0a8012b15b3c9c6ed167ea8c52a6de380180008abb000000101080a398758cab29173996a2e0388cd0741a95818040000000002192b88dc1fbe0300010007f960000516000000080007626573746f756c00043734363000060000000100174d6169736f6e657474652d6465732d656c657665757273000436343133e0f1ea1206000000020007596f6d696b6f61000435353034001600000004000a4461726b2d7975726973000432323433000600000005000a646961626f6c696b4949000433383437bf9dd810192b87dc1fbf0300010007f95f000406000000010006747974616e79000432363138000600000004000954726f7576746f7578000433343038000600000006000a616e746f6e2d73616d6100043136333980b489130600000007000b62726f7579657572626f790004313432330000040f5442507283804980000734001301010006ec18f718de21e821f221b420000501ffce8c0292fffa03f571ff04f571ff0592fffa00018c01000000055661686f7226420008000100030869af020000261164a204ffff03000000000c5b0102921feb02640042507283807b8000237dc0d3888000000000073400f501fd04000000000000000007a0000001ed03000d0005000001ec01000bfc040000000000000000000001eb01000bae030000000000000000000001ec04000efc040000000000000000000001eb03000dae030000000000000000000001ed03000dfd040000000000000000ffff00237dc0d38800000000000734001f05fb04000000000000000007a0000001ea02000c0000ffff00237dc0d3884000000000073400d801ae03000000000000000007a0000001eb02000c0003000001ea05000ffb040000000000000000000001ed02000cfd040000000000000000000001eb05000fae030000000000000000ffff00000817760007f9600000012c000224c7540000c31724c7610000c31a00000117760007f95f0000012c000224c7540000d11e24c7610000d12100000117760007fd9a0000013c000324c7e902000003db24c7d302000003da24c7b801000003d900000117760007ff020000006a000124c79d01000003dd00000017760007f9670000012c000124c7540000cb7c00000017760007ff030000006a000124c79d010000223900000017760007f9620000012c000124c7540000d09400000017760007f9630000012c000124c75400010ce200000000000000000001000cdc02ea02f90284039303a103ba03c903d703e303f1038004000cbc01bd01cb01cc01d901e501e801f301810282028f02900285592088dc1f00010007f9600f6616000000080007626573746f756c0004373436300085593388dc1f00010007f9600f66060000000100174d6169736f6e657474652d6465732d656c657665757273000436343133e0f1ea1285592088dc1f00010007f9600f6606000000020007596f6d696b6f610004353530340085592388dc1f00010007f9600f661600000004000a4461726b2d79757269730004323234330085592688dc1f00010007f9600f660600000005000a646961626f6c696b4949000433383437bf9dd81085594087dc1f00010007f95f118406000000010006747974616e7900043236313800cd19001254686520626c61636b20616e6420626c7565343b000ae9e91a0000000085593c87dc1f00010007f95f11840600000004000954726f7576746f757800043334303800ec1e000b54686972747920636c616ea41400ffffff1a0000000285592687dc1f00010007f95f0f660600000006000a616e746f6e2d73616d6100043136333980b4891385592487dc1f00010007f95f0f660600000007000b62726f7579657572626f7900043134323300";
        let house_properties_message = "9c760e4f1efc8c97ea31a2760800450002bcf5aa40003906e7d6ac41f3a5c0a8012b15b3c9c6ed1708efc52a70c180180008237e00000101080a3988eb76b293064f855921c6941c00010007db0d0f660600000008000848656c6c65626f7200043232333600855951c7941c00010007db0e118406000000010013756e70736575646f766f75736176657a64697400043538363700c51200154c6120636f6d7061676e69652064657320466f75782dc70300f17901040007030085592cc7941c00010007db0e0f66060000000200134d696e6b612d4d616972652d6441737472756200043832393700855924c7941c00010007db0e0f661600000003000b2d2d4d6f6f6e73686f742d00043434313900855920c7941c00010007db0e0f66040000000400074d696e6b612d3900043137383100855939c7941c00010007db0e11841600000006000945636174696e686f6f00043135383100b75900074e6f2d48616b694dac0300ba00041500859ef1855954c7941c00010007db0e118404000000080012436c616e2d6465732d53656c656e7974657300043536323900a34100194c65732041707072656e746973204176656e74757269657273368f0100edc61a1c004d1c1d855923c7941c00010007db0e0f661600000009000a70657469736f6c65696d00043431353800855925c7941c00010007db0e0f66060000000a000c6775697a696d616d6f756e650004373232350021c904ffffffff4d6d120003014601a501a400014233d2d30126000014619709006b486579205661686f722c20747520766575782061636865746572206f75206dc3aa6d652076656e64726520746573206b616d6173206175206d65696c6c65757220707269782073616e73207269737175652063276573742069636920717565206361207365207061737365660731df0008333773727973327a4261732fa024c000000b546574616c6f726164726100000aa5b9c2";
        let game_role_play_show_actor_message = "9c760e4f1efc8c97ea31a276080045000095f5c640003906e9e1ac41f3a5c0a8012b15b3c9c6ed170fddc52a70e480180008da0e00000101080a398909a0b2931f8c14e95e0f5442616e8ce024c00007340228060100076e8311ec09ed09f71fca07941b000501e5e1d102e6eeb403676e3d04b4bb8d05dff16e0001910100000007536978736f757326420008000000010c5b0109ce127200000042616e8ce033a000";
        let hex_streams = [
            chat_server_message_commerce,
            char_server_message_commerce_items,
            chat_server_message_recruitment,
            chat_server_message_recruitment_map,
            game_role_play_show_actor_message,
            house_properties_message,
            map_complementary_informations_data_message,
        ];
        let path = "tests/fixtures/".to_string();
        let path = Path::new(&path);
        let procol_manager = ProtocolManager::new(path).unwrap();
        let _ = Node::init_logger(path);

        for hex_stream in hex_streams.iter() {
            let hex = (0..hex_stream.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex_stream[i..i + 2], 16).unwrap())
                .collect::<Vec<u8>>();

            let header = PacketHeader::from_vec(&hex).unwrap();
            let metadata = PacketMetadata::from_buffer(header.body).unwrap();
            let mut parser = PacketParser::from_metadata(&metadata);

            let packet = parser.parse(&procol_manager).unwrap();
            info!("Packet: {:?}", packet);
        }
    }
}
