use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager, WebviewUrl, Wry};
use uuid::Uuid;

use crate::node::Node;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct WindowOptions {
    #[serde(skip)]
    pub id: Uuid,
    pub visible: bool,
    pub position: WindowType,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "camelCase")]
pub enum WindowType {
    Tab(Uuid),
    Window(WindowPosition),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Default for WindowPosition {
    fn default() -> Self {
        WindowPosition {
            x: 0,
            y: 0,
            width: 800,
            height: 600,
        }
    }
}

impl Default for WindowOptions {
    fn default() -> Self {
        WindowOptions::new(true, WindowType::Window(WindowPosition::default()))
    }
}

impl WindowOptions {
    pub fn new(visible: bool, position: WindowType) -> Self {
        WindowOptions {
            id: Uuid::new_v4(),
            visible,
            position,
        }
    }
}

pub struct WindowBuilder;

pub struct WindowBuilderOptions {
    pub options: WindowOptions,
    pub prefix: &'static str,
    pub webview: WebviewUrl,
}

impl WindowBuilder {
    pub fn prepare<'a>(
        node: &'a Node,
        options: WindowBuilderOptions,
    ) -> Option<tauri::WebviewWindowBuilder<'a, Wry, AppHandle>> {
        match options.options.position {
            WindowType::Tab(tab_id) => {
                return None;
            }
            _ => {}
        }
        let window_id = options.options.id;
        let handle = node.handle.as_ref().unwrap();

        let main_window = handle.get_webview_window("main").unwrap();
        let window = tauri::WebviewWindowBuilder::new(
            handle,
            format!("{}{}", options.prefix, window_id),
            options.webview,
        )
        .parent(&main_window)
        .expect("Failed to set parent window")
        .title(window_id.to_string())
        .hidden_title(true)
        .resizable(true)
        .visible(options.options.visible)
        .always_on_top(true);

        return match options.options.position {
            WindowType::Tab(..) => Some(window),
            WindowType::Window(position) => Some(
                window
                    .position(position.x.into(), position.y.into())
                    .inner_size(position.width.into(), position.height.into()),
            ),
        };
    }

    pub fn export<'a>(node: &'a Node, prefix: &'static str, window_id: Uuid) -> WindowOptions {
        let handle = node.handle.as_ref().unwrap();
        let window = handle
            .get_webview_window(format!("{}{}", prefix, window_id.to_string()).as_str())
            .unwrap();
        let position = window.outer_position().unwrap();
        let size = window.inner_size().unwrap();

        let options = WindowOptions {
            id: window_id,
            visible: window.is_visible().unwrap(),
            position: WindowType::Window(WindowPosition {
                x: position.x,
                y: position.y,
                width: size.width,
                height: size.height,
            }),
        };
        options
    }
}
