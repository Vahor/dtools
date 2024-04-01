use std::sync::Arc;

use crate::{
    features::{
        chat::config::ChatEvent,
        windows::{WindowBuilder, WindowBuilderOptions, WindowOptions},
    },
    node::Node,
    sniffer::{parser::packet::Packet, protocol::protocol::KnownEvent},
};
use tauri::{Manager, WindowEvent};
use tauri_specta::Event;
use tracing::{debug, info};
use uuid::Uuid;

use super::config::ChatViewsConfig;

#[derive(Debug, Clone)]
pub struct ChatFeature {
    node: Option<Arc<Node>>,
    config: Option<Arc<crate::config::Manager<ChatViewsConfig>>>,
}

const WINDOW_PREFIX: &str = "chat-";

impl ChatFeature {
    pub fn new() -> Self {
        ChatFeature {
            node: None,
            config: None,
        }
    }
    pub async fn set_node(&mut self, node: Arc<Node>) {
        self.node = Some(node);
        let dir_path = &self.node.as_ref().unwrap().data_dir;
        let manager =
            crate::config::Manager::<ChatViewsConfig>::new(dir_path, "chat.views.json").await;
        self.config = Some(manager.unwrap());
    }

    pub fn load_from_config(&mut self) {
        let tabs = self
            .config
            .as_ref()
            .unwrap()
            .config
            .read()
            .unwrap()
            .views
            .iter()
            .map(|(id, tab)| (id.clone(), tab.window.clone().unwrap()))
            .collect::<Vec<_>>();

        for (id, option) in tabs {
            self.create_window_with_options(id, option);
        }
    }

    fn build_window_label(&self, id: &str) -> String {
        format!("{}{}", WINDOW_PREFIX, id)
    }

    fn create_window_with_options(
        &mut self,
        id: String,
        options: WindowOptions,
    ) -> Option<tauri::WebviewWindow> {
        let node = self.node.as_ref().unwrap();

        let options = WindowBuilderOptions {
            options,
            window_label: self.build_window_label(&id),
            webview: tauri::WebviewUrl::App("features/chat".into()),
        };
        match WindowBuilder::prepare(node, options) {
            Some(builder) => {
                let window = builder.build().unwrap();
                self.register_window(&window);

                return Some(window);
            }
            None => {
                return None;
            }
        }
    }

    pub fn create_window(&mut self) {
        let options = WindowOptions::default();
        let id = Uuid::new_v4().to_string();
        self.create_window_with_options(id, options);
    }

    fn handle_subscription(&self) {
        let config = self.config.as_ref().unwrap().config.read().unwrap();
        let has_window = config.views.len() > 0;

        let node = self.node.as_ref().unwrap();
        let mut packet_listner = node.packet_listener.lock().unwrap();
        let events = [
            KnownEvent::ChatServerMessage.to_string(),
            KnownEvent::ChatServerWithObjectMessage.to_string(),
        ]
        .iter()
        .map(|name| {
            let id = node.protocol.get_protocol_id_by_class(name).unwrap();
            id
        })
        .collect::<Vec<_>>();

        if has_window {
            events.iter().for_each(|id| {
                let is_subscribed = packet_listner.has_subscriptions_for(id, WINDOW_PREFIX);
                if is_subscribed {
                    return;
                }

                packet_listner.subscribe(**id, WINDOW_PREFIX, move |packet, node| {
                    ChatFeature::listener(packet, node);
                });
            });
        } else {
            events.iter().for_each(|id| {
                let is_subscribed = packet_listner.has_subscriptions_for(id, WINDOW_PREFIX);
                if !is_subscribed {
                    return;
                }
                packet_listner.unsubscribe(id, WINDOW_PREFIX);
            });
        }
    }

    fn listener(packet: &Packet, node: &Node) {
        // iter windows
        // check if the window is subscribed to the event
        // if it is, send the packet to the window
        //
        let chat_feature = node.features.chat.read().unwrap();
        let config = chat_feature.config.as_ref().unwrap().config.read().unwrap();

        let handle = node.handle.as_ref().unwrap();

        let chat_event = ChatEvent::from_packet(&packet);
        let views = config
            .views
            .iter()
            .filter(|(_, tab)| {
                tab.filters
                    .as_ref()
                    .map_or(true, |filters| filters.evaluate(&chat_event))
            })
            .collect::<Vec<_>>();

        for (id, ..) in views.iter() {
            // TODO: handle tabs
            let window_label = format!("{}{}", WINDOW_PREFIX, id); // TODO: add function to get window label
            chat_event
                .clone()
                .emit_to(handle, window_label.as_str())
                .unwrap();
            debug!("Chat event sent to window {}", window_label);
        }
    }

    fn register_window(&mut self, window: &tauri::WebviewWindow) {
        let id = window.title().expect("Window title").to_string();
        info!("Chat window with id {} registered", id);
        self.save_window(&id);

        let config = self.config.as_ref().unwrap().clone();
        let self_ = self.clone();
        window.on_window_event(move |event| match event {
            WindowEvent::CloseRequested { .. } => {
                info!("Chat window closed");
                // Remove from config
                config
                    .update_config_sync(|config| {
                        let window_id = &id.to_string().replace(WINDOW_PREFIX, "");
                        config.views.remove(window_id);
                    })
                    .unwrap();
                self_.handle_subscription();
            }
            _ => {}
        });

        self.handle_subscription();
    }

    fn save_window(&mut self, window_id: &String) {
        let node = self.node.as_ref().unwrap();
        let window_option = WindowBuilder::export(node, window_id.to_string());

        let window_id = window_id.to_string().replace(WINDOW_PREFIX, "");
        self.config
            .as_mut()
            .unwrap()
            .update_config_sync(|config| {
                let tab = config.views.get_mut(&window_id.to_string());
                if let Some(tab) = tab {
                    tab.window = Some(window_option);
                } else {
                    // This method will be called when creating a new window
                    // so it's safe to use default value
                    let new_tab = super::config::ChatTabConfig {
                        options: super::config::ChatTabOptions::default(),
                        filters: None,
                        window: Some(window_option),
                    };
                    config.views.insert(window_id.to_string(), new_tab);
                }
            })
            .unwrap();
    }

    pub fn get_window_config(&self, window_id: &String) -> Option<super::config::ChatTabConfig> {
        let window_id = window_id.replace(WINDOW_PREFIX, "");
        let config = self.config.as_ref().unwrap().config.read().unwrap();
        let tab = config.views.get(&window_id);
        tab.cloned()
    }

    pub fn update_window_config(
        &self,
        window_id: &String,
        new_config: super::config::ChatTabConfig,
    ) {
        let window_id = window_id.replace(WINDOW_PREFIX, "");
        self.config
            .as_ref()
            .unwrap()
            .update_config_sync(|config| {
                let tab = config.views.get_mut(&window_id);
                if let Some(tab) = tab {
                    *tab = new_config;
                }
            })
            .unwrap();
    }
}
