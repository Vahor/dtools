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
    #[error("Missing header")]
    MissingHeader,
}

#[derive(Debug, Clone)]
pub struct PacketMetadata {
    pub direction: PacketDirection,
    pub data: Vec<u8>,
    pub id: u16,
}

impl PacketMetadata {
    pub fn from_buffer(buffer: &mut DataWrapper) -> Result<Self, ParseResult> {
        let data = &buffer.data;
        // Structure of a packet:
        // Ethernet header (14 bytes)
        // IP header (20 bytes)
        // TCP header (20 bytes)
        // Data

        if data.len() < 54 {
            // 14 + 20 + 20
            return Err(ParseResult::MissingHeader);
        }

        let eth_header_length = 14;
        let ip_header_length = ((data[eth_header_length] & 0x0F) as usize) * 4;
        let tcp_start = eth_header_length + ip_header_length;
        let tcp_header_length = ((data[tcp_start + 12] >> 4) as usize) * 4;
        let tcp_payload_start = tcp_start + tcp_header_length;

        if data.len() < tcp_start + 20 {
            // Packet is too short to contain a TCP header
            return Err(ParseResult::Invalid);
        }

        if data.len() < tcp_payload_start {
            // Packet is too short to contain a TCP payload
            return Err(ParseResult::Invalid);
        }

        let source_port = u16::from_be_bytes([data[tcp_start], data[tcp_start + 1]]);
        let destination_port = u16::from_be_bytes([data[tcp_start + 2], data[tcp_start + 3]]);

        // Skip to the PacketData
        let body = &data[tcp_payload_start..];

        //  [header sur 2 octets][taille du contenu sur 1, 2 ou 3 octets][contenu]
        // (id_du_message) << 2 + type de taille

        let header = u16::from_be_bytes([body[0], body[1]]);
        let id = header >> 2;
        let size_type = header & 0b11;
        dbg!(size_type);
        let (content_start_pos, content_size) = match size_type {
            1 => (4, u16::from_be_bytes([data[2], data[3]]) as usize), // 2 bytes
            _ => return Err(ParseResult::Invalid), // TODO: handle other size types
        };

        if body.len() < content_start_pos + content_size {
            // return Err(ParseResult::Incomplete);
        }

        let body = body[content_start_pos..].to_vec();

        // Get packet direction
        let in_port = 5555;
        let direction = if in_port == source_port {
            PacketDirection::In
        } else if in_port == destination_port {
            PacketDirection::Out
        } else {
            PacketDirection::Unknown
        };

        // decode body
        // let body_st = String::from_utf8_lossy(&body);
        // println!("Packet body: {:?}", body_st);

        Ok(PacketMetadata {
            direction,
            data: body,
            id,
        })
    }
}
