use std::sync::Arc;

use crate::{
    features::windows::{WindowBuilder, WindowBuilderOptions, WindowOptions},
    node::Node,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct ChatFeature {
    node: Option<Arc<Node>>,
    windows: Vec<Uuid>,
}

const WINDOW_PREFIX: &str = "chat-";

impl ChatFeature {
    pub fn new() -> Self {
        ChatFeature {
            node: None,
            windows: Vec::new(),
        }
    }
    pub fn set_node(&mut self, node: Arc<Node>) {
        self.node = Some(node);
    }

    pub fn load_from_config(&mut self) {
        let node = &self.node.as_ref().unwrap();
        let config = node.config.config.read().unwrap();
        for tab in config.features.chat.views.iter() {
            let (_, id) = self.create_window_with_options(tab.window.clone().unwrap());
            self.windows.push(id);
        }
    }

    fn create_window_with_options(
        &self,
        options: WindowOptions,
    ) -> (Option<tauri::WebviewWindow>, Uuid) {
        let node = self.node.as_ref().unwrap();

        let options = WindowBuilderOptions {
            options,
            prefix: WINDOW_PREFIX,
            webview: tauri::WebviewUrl::App("chat".into()),
        };
        let id = options.options.id;
        match WindowBuilder::prepare(node, options) {
            Some(builder) => {
                let window = builder.build().unwrap();
                return (Some(window), id);
            }
            None => {
                return (None, id);
            }
        }
    }

    pub fn create_window(&mut self) {
        let options = WindowOptions::default();
        let (_, id) = self.create_window_with_options(options);
        self.windows.push(id);
        self.save_window(id);
    }

    fn save_window(&self, window_id: Uuid) {
        let node = self.node.as_ref().unwrap();
        let window_option = WindowBuilder::export(node, WINDOW_PREFIX, window_id);

        node.config
            .update_config_sync(|config| {
                let chat_config = &mut config.features.chat;
                let tab = chat_config
                    .views
                    .iter_mut()
                    .find(|tab| tab.window.as_ref().unwrap().id == window_id);
                if let Some(tab) = tab {
                    tab.window = Some(window_option);
                } else {
                    // This method will be called when creating a new window
                    // so it's safe to use default value
                    chat_config.views.push(super::config::ChatTabConfig {
                        options: super::config::ChatTabOptions::default(),
                        filters: None,
                        window: Some(window_option),
                    });
                }
            })
            .unwrap();
    }

    fn remove_window(&mut self, window_id: Uuid) {
        let node = self.node.as_ref().unwrap();

        node.config
            .update_config_sync(|config| {
                let chat_config = &mut config.features.chat;
                chat_config
                    .views
                    .retain(|tab| tab.window.as_ref().unwrap().id != window_id);
            })
            .unwrap();
    }
}
