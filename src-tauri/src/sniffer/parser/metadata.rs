use anyhow::Result;
use thiserror::Error;

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
    MissingHeader(usize),
}

#[derive(Debug, Clone)]
pub struct PacketMetadata {
    pub data: Vec<u8>,
    pub id: u16,
    pub size: u16,
}

#[derive(Debug, Clone)]
pub struct PacketHeader {
    pub source_port: u16,
    pub destination_port: u16,
    pub source_ip: Vec<u8>,
    pub seq_num: u16,
    pub tcp_payload_start: usize,
    pub body: Vec<u8>,
}

impl PacketHeader {
    pub fn from_vec(data: &Vec<u8>) -> Result<Self, ParseResult> {
        // Structure of a packet:
        // Ethernet header (14 bytes)
        // IP header (20 bytes)
        // TCP header (20 bytes)
        // Data

        if data.len() < 54 {
            return Err(ParseResult::MissingHeader(data.len()));
        }

        let eth_header_length = 14;
        let ip_header_length = ((data[eth_header_length] & 0x0F) as usize) * 4;
        let seq_num =
            u16::from_be_bytes([data[eth_header_length + 4], data[eth_header_length + 5]]);
        let ip_source = &data[eth_header_length + 12..eth_header_length + 16];
        let tcp_start = eth_header_length + ip_header_length;
        let tcp_header_length = ((data[tcp_start + 12] >> 4) as usize) * 4;
        let tcp_payload_start = tcp_start + tcp_header_length;

        let source_port = u16::from_be_bytes([data[tcp_start], data[tcp_start + 1]]);
        let destination_port = u16::from_be_bytes([data[tcp_start + 2], data[tcp_start + 3]]);

        if data.len() < tcp_payload_start {
            // Packet is too short to contain a TCP payload
            return Err(ParseResult::Invalid);
        }
        Ok(PacketHeader {
            source_port,
            destination_port,
            source_ip: ip_source.to_vec(),
            seq_num,
            tcp_payload_start,
            body: data[tcp_payload_start..].to_vec(),
        })
    }
}

impl PacketMetadata {
    pub fn from_buffer(body: Vec<u8>) -> Result<Self, ParseResult> {
        if body.len() < 3 {
            return Err(ParseResult::Invalid);
        }

        //  [header sur 2 octets][taille du contenu sur 1, 2 ou 3 octets][contenu]
        // (id_du_message) << 2 + type de taille

        let header = u16::from_be_bytes([body[0], body[1]]);
        let id = header >> 2;
        let size_type = header & 0b11;
        let content_size = match size_type {
            0 => 0,                                                           // 0 bytes
            1 => u32::from_be_bytes([0, 0, 0, body[2]]) as usize,             // 1 bytes
            2 => u32::from_be_bytes([0, 0, body[2], body[3]]) as usize,       // 2 bytes
            3 => u32::from_be_bytes([0, body[2], body[3], body[4]]) as usize, // 3 bytes
            _ => return Err(ParseResult::Invalid),
        };

        let content_start = 2 + size_type as usize;
        if body.len() < (content_start + content_size) {
            return Err(ParseResult::Incomplete);
        }

        let body = body[content_start..content_start + content_size].to_vec();

        Ok(PacketMetadata {
            data: body,
            id,
            size: content_size as u16,
        })
    }
}
