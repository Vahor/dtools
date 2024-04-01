use std::{collections::HashMap, sync::Arc};

use crate::{
    features::chat::config::ChatEvent,
    node::Node,
    sniffer::{parser::packet::Packet, protocol::protocol::KnownEvent},
};
use tauri_specta::Event;
use tracing::debug;
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

    fn build_window_label(&self, id: &str) -> String {
        format!("{}{}", WINDOW_PREFIX, id)
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
            let window_label = chat_feature.build_window_label(id);
            chat_event
                .clone()
                .emit_to(handle, window_label.as_str())
                .unwrap();
            debug!("Chat event sent to window {}", window_label);
        }
    }

    pub fn create_tab(&mut self, config: super::config::ChatTabConfig) {
        let id = Uuid::new_v4().to_string();
        let window_label = self.build_window_label(&id);
        self.update_tab_config(&window_label, config);
        self.handle_subscription();
    }

    pub fn delete_tab(&mut self, window_id: &String) {
        let window_id = window_id.replace(WINDOW_PREFIX, "");
        self.config
            .as_ref()
            .unwrap()
            .update_config_sync(|config| {
                config.views.remove(&window_id);
            })
            .unwrap();
        self.handle_subscription();
    }

    pub fn get_tab_config(&self, window_id: &String) -> Option<super::config::ChatTabConfig> {
        let window_id = window_id.replace(WINDOW_PREFIX, "");
        let config = self.config.as_ref().unwrap().config.read().unwrap();
        let tab = config.views.get(&window_id);
        tab.cloned()
    }

    pub fn list_tabs(&self) -> HashMap<String, super::config::ChatTabConfig> {
        let config = self.config.as_ref().unwrap().config.read().unwrap();
        config.views.clone()
    }

    pub fn update_tab_config(&self, window_id: &String, new_config: super::config::ChatTabConfig) {
        let window_id = window_id.replace(WINDOW_PREFIX, "");
        self.config
            .as_ref()
            .unwrap()
            .update_config_sync(|config| {
                let tab = config.views.get_mut(&window_id);
                if let Some(tab) = tab {
                    *tab = new_config;
                } else {
                    config.views.insert(window_id.to_string(), new_config);
                }
            })
            .unwrap();
    }
}
