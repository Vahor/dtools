use extractor::DataExtractor;
use once_cell::sync::OnceCell;
use sniffer::PacketReader;
use tauri::AppHandle;

#[derive(Debug)]
pub struct App {
    pub packet: PacketReader,
    pub extractor: DataExtractor,
}

static INSTANCE: OnceCell<App> = OnceCell::new();

impl App {
    pub fn new(handler: &AppHandle) -> App {
        App {
            // Must be created before PacketReader
            extractor: DataExtractor::new(handler).expect("Failed to create DataExtractor"),
            packet: PacketReader::new(handler).expect("Failed to create PacketReader"),
        }
    }

    pub fn init(handler: &AppHandle) {
        if INSTANCE.get().is_some() {
            return;
        }

        INSTANCE
            .set(App::new(handler))
            .expect("This shoduld never happen");
    }

    pub fn instance() -> &'static App {
        INSTANCE.get().expect("App not initialized")
    }

    pub fn run(&self, name: &str) -> String {
        return format!("Hello, {}!", name);
    }
}
