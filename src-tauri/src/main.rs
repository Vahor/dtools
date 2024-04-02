// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use node::Node;
use tauri::{Manager, WindowEvent};
use tauri_specta::ts;
use tracing::{debug, error, info};

use crate::features::chat::config::ChatEvent;

pub mod config;
pub mod constants;
pub mod downloader;
pub mod features;
pub mod node;
pub mod sniffer;

fn fix_specta(path: &str) {
    // replace all occurence of "plugin:tauri-specta" in the file
    // This is because the plugin is not yet released and the permissions are not working
    // TODO: This is a temporary fix

    let content = std::fs::read_to_string(path).unwrap();
    let content = content.replace("plugin:tauri-specta|", "");
    std::fs::write(path, content).unwrap();
    println!("Fixed specta");
}

#[tauri::command(async)]
#[specta::specta]
async fn app_ready(handle: tauri::AppHandle) {
    info!("App is ready");
    let main_window = handle.get_webview_window("main").unwrap();
    // TODO: does not work, maybe because tauri beta ?
    main_window.show().unwrap();
}

#[tauri::command]
#[specta::specta]
fn create_chat_tab(
    state: tauri::State<'_, Arc<Node>>,
    config: features::chat::config::ChatTabConfig,
) -> String {
    // TODO: specta issue, we can't move the function in a separate file
    let mut chat = state.features.chat.write().unwrap();
    chat.create_tab(config)
}

#[tauri::command]
#[specta::specta]
fn delete_chat_tab(state: tauri::State<'_, Arc<Node>>, window_id: String) {
    let mut chat = state.features.chat.write().unwrap();
    chat.delete_tab(&window_id);
}

#[tauri::command]
#[specta::specta]
fn get_chat_tab_config(
    state: tauri::State<'_, Arc<Node>>,
    window_id: String,
) -> Option<features::chat::config::ChatTabConfig> {
    let chat = state.features.chat.read().unwrap();
    chat.get_tab_config(&window_id)
}

#[tauri::command]
#[specta::specta]
fn update_chat_tab_config(
    state: tauri::State<'_, Arc<Node>>,
    window_id: String,
    config: features::chat::config::ChatTabConfig,
) {
    let chat = state.features.chat.write().unwrap();
    chat.update_tab_config(&window_id, config);
}

#[tauri::command]
#[specta::specta]
fn list_chat_tabs(
    state: tauri::State<'_, Arc<Node>>,
) -> std::collections::HashMap<String, features::chat::config::ChatTabConfig> {
    let chat = state.features.chat.read().unwrap();
    chat.list_tabs()
}

#[tauri::command]
#[specta::specta]
fn set_active_chat_tab(state: tauri::State<'_, Arc<Node>>, window_id: Option<String>) {
    let mut chat = state.features.chat.write().unwrap();
    chat.set_active_tab(window_id);
}

#[tauri::command]
#[specta::specta]
fn get_last_open_chat_tab(state: tauri::State<'_, Arc<Node>>) -> Option<String> {
    let chat = state.features.chat.read().unwrap();
    chat.get_last_active_tab()
}

#[tauri::command]
#[specta::specta]
fn get_global_config(state: tauri::State<'_, Arc<Node>>) -> config::NodeConfig {
    let config = state.config.config.read().unwrap();
    debug!("Config: {:?}", *config);
    config.clone()
}

#[tauri::command]
#[specta::specta]
fn get_last_packet_timestamp(state: tauri::State<'_, Arc<Node>>) -> u128 {
    let packet_listener = state.packet_listener.lock().unwrap();
    let last_packet_time = packet_listener.last_packet_time.read().unwrap();
    last_packet_time.clone()
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
                create_chat_tab,
                update_chat_tab_config,
                get_chat_tab_config,
                list_chat_tabs,
                get_global_config,
                get_last_packet_timestamp,
                set_active_chat_tab,
                get_last_open_chat_tab,
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
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_fs::init())
        // TODO: use tauri-specta when v2 is released
        .invoke_handler(tauri::generate_handler![
            app_ready,
            create_chat_tab,
            update_chat_tab_config,
            get_chat_tab_config,
            list_chat_tabs,
            get_global_config,
            get_last_packet_timestamp,
            set_active_chat_tab,
            get_last_open_chat_tab,
        ]);

    app.run(tauri::generate_context!())
        .expect("error while running tauri application");
}
