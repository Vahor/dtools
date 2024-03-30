use lazy_static::lazy_static;
use regex::Regex;

pub const CONFIG_FILE_NAME: &str = "config.json";
pub const EXTRACTOR_DIR: &str = "dofus/datafus";
pub const DATA_URL: &str = "https://github.com/bot4dofus/Datafus/releases";

lazy_static! {
    pub static ref VERSION_REGEX: Regex = Regex::new(r"(\d+\.\d+\.\d+)").unwrap();
    pub static ref TRANSLATIONS_DIR: String = format!("{}/translations.json", EXTRACTOR_DIR);
    pub static ref ENTITIES_DIR: String = format!("{}/entities.json", EXTRACTOR_DIR);
    pub static ref EVENTS_FILE: String = format!("{}/events.json", EXTRACTOR_DIR);
}
