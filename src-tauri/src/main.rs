// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use node::Node;
use tauri::Manager;
use tracing::{error, info};

pub mod config;
pub mod constants;
pub mod downloader;
pub mod node;
pub mod sniffer;

#[tauri::command]
fn greet(state: tauri::State<'_, Arc<Node>>, name: &str) -> String {
    state.greet(name)
}

fn main() {
    let app = tauri::Builder::default();

    info!("Starting Node...");

    let app = app
        .plugin(tauri_plugin_shell::init())
        // .plugin(tauri_plugin_store::init())
        .plugin(tauri_plugin_fs::init());

    let app = app.setup(move |app| {
        // let splashscreen_window = app.get_window("splashscreen").unwrap();
        let main_window = app.get_webview_window("main").unwrap();
        let handle = app.handle();
        tauri::async_runtime::block_on(async {
            let data_dir = handle.path().app_data_dir().unwrap();
            let node = node::Node::new(data_dir, Some(handle.clone()), true).await;
            if let Err(e) = node {
                error!("Failed to initialize node: {:?}", e);
                handle.exit(1);
                return;
            }
            app.manage(node.unwrap());
            main_window.show().unwrap();
        });

        Ok(())
    });

    app.invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
