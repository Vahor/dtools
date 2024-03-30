use anyhow::Result;
use thiserror::Error;

use super::wrapper::DataWrapper;

#[derive(Debug, Clone)]
pub enum PacketDirection {
    In,
    Out,
    Unknown,
}

#[derive(Debug, Error)]
pub enum ParseResult {
    #[error("Packet is invalid")]
    Invalid,
    #[error("Packet is incomplete")]
    Incomplete,
}

#[derive(Debug, Clone)]
pub struct PacketMetadata {
    pub direction: PacketDirection,
    pub data: Vec<u8>,
    pub id: u16,
}

impl PacketMetadata {
    pub fn from_buffer(buffer: &mut DataWrapper) -> Result<Self, ParseResult> {
        // Convert to hex
        let hex = buffer
            .get_remaining()
            .iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<String>>();

        if hex.len() < 54usize {
            return Err(ParseResult::Invalid);
        }

        // hex contains Ethernet frame, Internet Protocol version 4 (IPv4) packet, Transmission Control Protocol (TCP) segment, and HTTP request

        let tcp = &hex[34..];
        let source_port = &tcp[0..2];
        let source_port = u16::from_str_radix(&source_port.join(""), 16).unwrap();
        let destination_port = &tcp[2..4];
        let destination_port = u16::from_str_radix(&destination_port.join(""), 16).unwrap();

        // Get packet direction
        let in_port = 5555;
        let direction = if in_port == source_port {
            PacketDirection::In
        } else if in_port == destination_port {
            PacketDirection::Out
        } else {
            PacketDirection::Unknown
        };

        // Skip to the PacketData
        let packet_data = &hex[54..];

        // Convert hex to u8
        let mut packet_data_u8 = Vec::new();
        for i in 0..packet_data.len() {
            let byte = u8::from_str_radix(&packet_data[i], 16).unwrap();
            packet_data_u8.push(byte);
        }

        if packet_data_u8.len() < 1 {
            // header and length
            return Err(ParseResult::Invalid);
        }

        let mut buffer = DataWrapper::new(packet_data_u8);

        let header = buffer.read_unsigned_short();
        let len = buffer.read((header & 3) as usize);
        if len.len() == 0 {
            return Err(ParseResult::Invalid);
        }
        let len = u32::from_be_bytes([0, 0, 0, len[0]]);
        let id = (header >> 2) as u16;

        if buffer.remaining() < (len as usize) {
            return Err(ParseResult::Incomplete);
        }

        let data = buffer.read(len as usize);

        Ok(PacketMetadata {
            direction,
            data,
            id,
        })
    }
}
