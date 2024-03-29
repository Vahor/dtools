use std::sync::Mutex;

use extractor::DataExtractor;
use sniffer::PacketReader;
use tauri::AppHandle;

#[derive(Debug)]
pub struct AppState {
    pub packet: PacketReader,
    pub extractor: DataExtractor,
}

pub type WrappedState = Mutex<Option<AppState>>;

impl AppState {
    pub fn new(handle: &AppHandle) -> AppState {
        AppState {
            // Must be created before PacketReader
            extractor: DataExtractor::new(handle).expect("Failed to create DataExtractor"),
            packet: PacketReader::new(handle).expect("Failed to create PacketReader"),
        }
    }

    pub fn greet(&self, name: &str) -> String {
        return format!("Hello, {}!", name);
    }
}
