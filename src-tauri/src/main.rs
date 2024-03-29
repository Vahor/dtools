// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app;
use app::App;

#[tauri::command]
fn greet(name: &str) -> String {
    App::instance().run(name)
}

fn main() {
    App::init();

    let app = tauri::Builder::default();

    let app = app
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init());

    app.invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
