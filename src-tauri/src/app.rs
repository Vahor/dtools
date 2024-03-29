use once_cell::sync::OnceCell;
use sniffer::PacketReader;
use tauri::AppHandle;

#[derive(Debug)]
pub struct App {
    pub packet: PacketReader,
}

static INSTANCE: OnceCell<App> = OnceCell::new();

impl App {
    pub fn new(handler: &AppHandle) -> App {
        App {
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
