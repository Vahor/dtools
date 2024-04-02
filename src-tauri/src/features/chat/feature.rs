use std::io::prelude::*;
use std::{
    collections::HashMap,
    fs::{self, OpenOptions},
    path::PathBuf,
    sync::Arc,
};

use crate::{
    features::chat::config::ChatEvent,
    node::Node,
    sniffer::{parser::packet::Packet, protocol::protocol::KnownEvent},
};
use tauri_plugin_notification::NotificationExt;
use tauri_specta::Event;
use tracing::debug;
use uuid::Uuid;

use super::config::ChatViewsConfig;

#[derive(Debug, Clone)]
pub struct ChatFeature {
    node: Option<Arc<Node>>,
    dir_path: Option<PathBuf>,
    config: Option<Arc<crate::config::Manager<ChatViewsConfig>>>,
    pub active_tab: Option<String>,
}

const LISTENER_ID: &str = "chat";

impl ChatFeature {
    pub fn new() -> Self {
        ChatFeature {
            node: None,
            config: None,
            dir_path: None,
            active_tab: None,
        }
    }

    pub fn get_last_active_tab(&self) -> Option<String> {
        let config = self.config.as_ref().unwrap().config.read().unwrap();
        config.last_tab_id.clone()
    }

    pub async fn set_node(&mut self, node: Arc<Node>) {
        self.node = Some(node);
        let dir_path = &self.node.as_ref().unwrap().data_dir.join("features/chat");
        fs::create_dir_all(dir_path).unwrap();
        let manager = crate::config::Manager::<ChatViewsConfig>::new(dir_path, "tabs.json").await;
        self.config = Some(manager.unwrap());
        self.dir_path = Some(dir_path.to_path_buf());
        self.init_subscription();
    }

    fn append_history(&self, id: &str, event: &ChatEvent) {
        let to_str = serde_json::to_string(event).unwrap();
        let dir_path = self.dir_path.as_ref().unwrap().join("history");
        let history_path = dir_path.join(format!("{}.jsonl", id));
        fs::create_dir_all(&dir_path).unwrap();

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(history_path)
            .unwrap();

        writeln!(file, "{}", to_str).unwrap();
    }

    fn init_subscription(&self) {
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

        events.iter().for_each(|id| {
            let is_subscribed = packet_listner.has_subscriptions_for(id, LISTENER_ID);
            if is_subscribed {
                return;
            }

            packet_listner.subscribe(**id, LISTENER_ID, move |packet, node| {
                ChatFeature::listener(packet, node);
            });
        });
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
        let active_window = chat_feature.active_tab.clone();

        for (id, tab) in views.iter() {
            let is_active = active_window.as_ref().map_or(false, |active| active == *id);
            debug!(
                "tab id: {}, active_id: {}",
                id,
                active_window.as_deref().unwrap_or("")
            );
            let has_notification = tab.options.notify;
            let is_persistent = tab.options.keep_history;

            let send_notification = (is_persistent && has_notification) || is_active;

            if is_persistent {
                chat_feature.append_history(&id, &chat_event);
            }

            if send_notification {
                handle
                    .notification()
                    .builder()
                    .title(format!("{} in {}", "New chat message", tab.name))
                    .show()
                    .unwrap();
            }

            if is_active {
                chat_event.clone().emit(handle).unwrap();
                continue;
            }
        }
    }

    pub fn create_tab(&mut self, config: super::config::ChatTabConfig) -> String {
        let id = Uuid::new_v4().to_string();
        self.update_tab_config(&id, config);
        id
    }

    pub fn set_active_tab(&mut self, tab_id: Option<String>) {
        if let Some(tab_id) = tab_id {
            self.active_tab = Some(tab_id.clone());
            let config = self.config.as_ref().unwrap();
            config
                .update_config_sync(|config| {
                    config.last_tab_id = Some(tab_id);
                })
                .unwrap();
        } else {
            self.active_tab = None;
        }
    }

    pub fn delete_tab(&mut self, window_id: &String) {
        self.config
            .as_ref()
            .unwrap()
            .update_config_sync(|config| {
                config.views.remove(window_id);
            })
            .unwrap();
    }

    pub fn get_tab_config(&self, window_id: &String) -> Option<super::config::ChatTabConfig> {
        let config = self.config.as_ref().unwrap().config.read().unwrap();
        let tab = config.views.get(window_id);
        tab.cloned()
    }

    pub fn list_tabs(&self) -> HashMap<String, super::config::ChatTabConfig> {
        let config = self.config.as_ref().unwrap().config.read().unwrap();
        config.views.clone()
    }

    pub fn update_tab_config(&self, window_id: &String, new_config: super::config::ChatTabConfig) {
        self.config
            .as_ref()
            .unwrap()
            .update_config_sync(|config| {
                let tab = config.views.get_mut(window_id);
                if let Some(tab) = tab {
                    *tab = new_config;
                } else {
                    config.views.insert(window_id.to_string(), new_config);
                }
            })
            .unwrap();
    }
}
