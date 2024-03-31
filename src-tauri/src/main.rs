// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use node::Node;
use tauri::Manager;
use tauri_specta::*;
use tracing::{error, info};

pub mod config;
pub mod constants;
pub mod downloader;
pub mod features;
pub mod node;
pub mod sniffer;

#[tauri::command]
#[specta::specta]
fn greet(state: tauri::State<'_, Arc<Node>>, name: &str) -> String {
    state.greet(name)
}

#[tauri::command(async)]
#[specta::specta]
async fn app_ready(handle: tauri::AppHandle) {
    info!("App is ready");
    let main_window = handle.get_webview_window("main").unwrap();
    main_window.show().unwrap();
}

fn main() {
    let app = tauri::Builder::default();

    info!("Starting Node...");

    let specta_plugin = {
        let specta_builder = ts::builder()
            .commands(tauri_specta::collect_commands![greet, app_ready])
            .config(specta::ts::ExportConfig::default().formatter(specta::ts::formatter::prettier));

        #[cfg(debug_assertions)]
        let specta_builder = specta_builder.path("../src-ui/bindings.ts");

        specta_builder.into_plugin()
    };

    let app = app
        .plugin(specta_plugin)
        .plugin(tauri_plugin_shell::init())
        // .plugin(tauri_plugin_store::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_fs::init());

    let app = app.setup(move |app| {
        // let splashscreen_window = app.get_window("splashscreen").unwrap();
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
        });

        Ok(())
    });

    app.invoke_handler(tauri::generate_handler![greet, app_ready,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
