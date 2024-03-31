use std::sync::Arc;

use tauri::Manager;
use tracing::error;

use crate::node::Node;

use super::config::{ChatTabConfig, ChatTabOptions};

#[derive(Debug)]
pub struct ChatFeature {
    node: Option<Arc<Node>>,
}

impl ChatFeature {
    pub fn new() -> Self {
        ChatFeature { node: None }
    }
    pub fn set_node(&mut self, node: Arc<Node>) {
        self.node = Some(node);
    }

    pub fn create_window(&mut self, options: ChatTabOptions) {
        let mut config = ChatTabConfig::default();
        config.options = options;

        let window_name = config.id.to_string();
        let node = self.node.as_ref().unwrap();
        let handle = node.handle.as_ref().unwrap();

        tauri::WebviewWindowBuilder::new(
            handle,
            window_name.clone(),
            tauri::WebviewUrl::App("/chat".parse().unwrap()),
        )
        .title("Chat")
        .resizable(true)
        .always_on_top(true)
        .build()
        .unwrap();

        self.update_window(window_name, config.options);
    }

    pub fn update_window(&mut self, window_name: String, options: ChatTabOptions) {
        let node = self.node.as_ref().unwrap();
        let handle = node.handle.as_ref().unwrap();

        let window = handle.get_webview_window(&window_name).unwrap();
        match window.is_visible() {
            Ok(visible) => {
                if visible && !options.visible {
                    window.hide().unwrap();
                } else if !visible && options.visible {
                    window.show().unwrap();
                }
            }
            Err(e) => {
                error!("Failed to get window visibility: {:?}", e);
            }
        }
    }
}
