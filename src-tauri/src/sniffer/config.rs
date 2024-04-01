use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type)]
pub struct NetworkConfig {
    pub port: u16,
    pub interface: String,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        NetworkConfig {
            port: 5555,
            interface: "en0".to_string(),
        }
    }
}
