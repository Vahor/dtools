// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use node::Node;
use tauri::{Manager, WindowEvent};
use tauri_specta::ts;
use tracing::{error, info};

use crate::features::chat::config::ChatEvent;

pub mod config;
pub mod constants;
pub mod downloader;
pub mod features;
pub mod node;
pub mod sniffer;

#[tauri::command(async)]
#[specta::specta]
async fn app_ready(handle: tauri::AppHandle) {
    info!("App is ready");
    let main_window = handle.get_webview_window("main").unwrap();
    main_window.show().unwrap();
}

fn fix_specta(path: &str) {
    // replace all occurence of "plugin:tauri-specta" in the file
    // This is because the plugin is not yet released and the permissions are not working
    // TODO: This is a temporary fix

    let content = std::fs::read_to_string(path).unwrap();
    let content = content.replace("plugin:tauri-specta|", "");
    std::fs::write(path, content).unwrap();
    println!("Fixed specta");
}

#[tauri::command]
#[specta::specta]
fn create_chat_window(state: tauri::State<'_, Arc<Node>>) {
    // TODO: specta issue, we can't move the function in a separate file
    let mut chat = state.features.chat.write().unwrap();
    chat.create_window();
    info!("Chat window created");
}

fn main() {
    let app = tauri::Builder::default();

    info!("Starting Node...");

    // TODO: use plugin when v2 is released
    let specta_plugin = {
        let specta_builder = ts::builder()
            .events(tauri_specta::collect_events![ChatEvent])
            .commands(tauri_specta::collect_commands![
                app_ready,
                create_chat_window,
            ])
            .config(
                specta::ts::ExportConfig::default()
                    .bigint(specta::ts::BigIntExportBehavior::BigInt)
                    .formatter(specta::ts::formatter::prettier),
            );

        let path = "../src/commands.ts";
        #[cfg(debug_assertions)]
        let specta_builder = specta_builder.path(path);

        let plugin = specta_builder.into_plugin();

        #[cfg(debug_assertions)]
        fix_specta(path);

        plugin
    };

    let app = app
        .setup(move |app| {
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
        })
        .on_window_event(move |window, event| match event {
            WindowEvent::CloseRequested { .. } => {
                if window.label() == "main" {
                    info!("Main window closed");
                    window.app_handle().exit(0);
                }
            }
            _ => {}
        });

    let app = app
        .plugin(specta_plugin)
        .plugin(tauri_plugin_shell::init())
        // .plugin(tauri_plugin_store::init())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_fs::init())
        // TODO: use tauri-specta when v2 is released
        .invoke_handler(tauri::generate_handler![app_ready, create_chat_window]);

    app.run(tauri::generate_context!())
        .expect("error while running tauri application");
}
