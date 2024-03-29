use anyhow::Result;

pub mod protocol;

#[derive(Debug)]
pub struct PacketReader {
    pub whitelist: Vec<String>,
}

impl PacketReader {
    pub fn new() -> Result<PacketReader> {
        let instance = PacketReader { whitelist: vec![] };

        return Ok(instance);
    }

    pub fn sniff(&self, url: &str) -> bool {
        println!("Sniffing: {}", url);
        true
    }
}
