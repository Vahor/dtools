use once_cell::sync::OnceCell;

#[derive(Debug)]
pub struct App {}

static INSTANCE: OnceCell<App> = OnceCell::new();

impl App {
    pub fn new() -> App {
        App {}
    }

    pub fn init() {
        if INSTANCE.get().is_some() {
            return;
        }
        INSTANCE.set(App::new()).expect("App already initialized");
    }

    pub fn instance() -> &'static App {
        INSTANCE.get().expect("App not initialized")
    }

    pub fn run(&self, name: &str) -> String {
        return format!("Hello, {}!", name);
    }
}
