use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, WebviewUrl, Wry};
use uuid::Uuid;

use crate::node::Node;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WindowOptions {
    #[serde(rename = "type")]
    pub type_: WindowType,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "camelCase")]
pub enum WindowType {
    Tab(Uuid),
    Window,
}

impl Default for WindowOptions {
    fn default() -> Self {
        WindowOptions::new(WindowType::Window)
    }
}

impl WindowOptions {
    pub fn new(type_: WindowType) -> Self {
        WindowOptions { type_ }
    }
}

pub struct WindowBuilder;

pub struct WindowBuilderOptions {
    pub options: WindowOptions,
    pub webview: WebviewUrl,
    pub window_label: String,
}

impl WindowBuilder {
    pub fn prepare<'a>(
        node: &'a Node,
        options: WindowBuilderOptions,
    ) -> Option<tauri::WebviewWindowBuilder<'a, Wry, AppHandle>> {
        match options.options.type_ {
            WindowType::Tab(..) => {
                return None;
            }
            _ => {}
        }
        let handle = node.handle.as_ref().unwrap();

        let window =
            tauri::WebviewWindowBuilder::new(handle, &options.window_label, options.webview)
                .title(&options.window_label)
                .hidden_title(true)
                .resizable(true)
                .always_on_top(true);

        return Some(window);
    }

    pub fn export<'a>(node: &'a Node, window_label: String) -> WindowOptions {
        let handle = node.handle.as_ref().unwrap();
        let _window = handle.get_webview_window(&window_label).unwrap();

        let options = WindowOptions {
            type_: WindowType::Window, // TODO: handle tabs
        };
        options
    }
}
