// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod state;
use std::sync::Mutex;

use state::{AppState, WrappedState};
use tauri::{Manager, State};

#[tauri::command]
fn greet(state: State<WrappedState>, name: &str) -> String {
    let state = state.lock().unwrap();

    if let Some(ref state) = *state {
        state.greet(name)
    } else {
        "Bug".to_string()
    }
}

fn main() {
    let app = tauri::Builder::default();

    let app = app
        .plugin(tauri_plugin_shell::init())
        // .plugin(tauri_plugin_store::init())
        .plugin(tauri_plugin_fs::init());

    let app = app.manage(Mutex::new(None::<AppState>));

    let app = app.setup(move |app| {
        let handle = app.handle(); // TODO: remove clone
        let state = &handle.state::<WrappedState>();
        let new_state = AppState::new(&handle);

        *state.lock().unwrap() = Some(new_state);
        Ok(())
    });

    app.invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
