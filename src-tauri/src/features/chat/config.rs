use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::sniffer::parser::packet::Packet;

#[derive(Clone, Serialize, Deserialize, Debug, specta::Type)]
pub struct ChatViewsConfig {
    #[serde(flatten)]
    pub views: HashMap<String, ChatTabConfig>,
}

#[derive(Clone, Serialize, Deserialize, Debug, specta::Type)]
pub struct ChatTabConfig {
    pub name: String,
    pub options: ChatTabOptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ChatTabFilterTree>,
    pub order: u8,
}

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub struct ChatTabOptions {
    pub persistent: bool,
    pub notify: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
pub struct ChatEvent {
    pub channel: u8,
    pub sender_name: String,
    pub content: String,
    pub timestamp: u32,
}

impl ChatEvent {
    pub fn from_packet(packet: &Packet) -> Self {
        ChatEvent {
            channel: packet.data.get("channel").unwrap().as_u64().unwrap() as u8,
            sender_name: packet
                .data
                .get("senderName")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            content: packet
                .data
                .get("content")
                .unwrap()
                .as_str()
                .unwrap()
                .to_string(),
            timestamp: packet.data.get("timestamp").unwrap().as_u64().unwrap() as u32,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum ChatTabFilterTree {
    And(Vec<ChatTabFilterTree>),
    Or(Vec<ChatTabFilterTree>),

    #[serde(untagged)]
    Filter(ChatTabFilterType),
}

impl ChatTabFilterTree {
    pub fn evaluate(&self, data: &ChatEvent) -> bool {
        match self {
            ChatTabFilterTree::And(filters) => filters.iter().all(|filter| filter.evaluate(data)),
            ChatTabFilterTree::Or(filters) => filters.iter().any(|filter| filter.evaluate(data)),
            ChatTabFilterTree::Filter(filter) => match filter {
                ChatTabFilterType::Channel(channel) => data.channel == *channel,
                ChatTabFilterType::Player(player) => data.sender_name == *player,
                ChatTabFilterType::Word(word) => data.content.contains(word),
                // ChatTabFilterType::Item(item) => data.objects.as_ref().unwrap().contains_key(item),
                _ => false,
            },
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, specta::Type)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "camelCase")]
pub enum ChatTabFilterType {
    Channel(u8),
    Player(String),
    Word(String),
    Item(u32),
}

impl Default for ChatViewsConfig {
    fn default() -> Self {
        ChatViewsConfig {
            views: HashMap::new(),
        }
    }
}

impl Default for ChatTabOptions {
    fn default() -> Self {
        ChatTabOptions {
            persistent: false,
            notify: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let config = ChatViewsConfig::default();
        assert_eq!(config.views.len(), 0);
    }

    #[test]
    fn test_deserialize() {
        let config = r#"{}"#;
        let config: ChatViewsConfig = serde_json::from_str(config).unwrap();
        assert_eq!(config.views.len(), 0);
    }

    #[test]
    fn test_serialize() {
        let config = ChatViewsConfig::default();
        let config = serde_json::to_string(&config).unwrap();
        assert_eq!(config, r#"{}"#);

        let config = ChatViewsConfig {
            views: HashMap::from([(
                "test".to_string(),
                ChatTabConfig {
                    name: "test".to_string(),
                    order: 0,
                    options: ChatTabOptions {
                        persistent: true,
                        notify: true,
                    },
                    filters: Some(ChatTabFilterTree::Filter(ChatTabFilterType::Channel(1))),
                },
            )]),
        };

        let config = serde_json::to_string(&config).unwrap();
        assert_eq!(
            config,
            r#"{"test":{"persistent":true,"notify":true,"filters":{"type":"channel","value":1},"order":0}}"#
        );

        let config = ChatViewsConfig {
            views: HashMap::from([(
                "test2".to_string(),
                ChatTabConfig {
                    name: "test2".to_string(),
                    order: 0,
                    options: ChatTabOptions {
                        persistent: true,
                        notify: true,
                    },
                    filters: Some(ChatTabFilterTree::And(vec![
                        ChatTabFilterTree::Filter(ChatTabFilterType::Channel(1)),
                        ChatTabFilterTree::Filter(ChatTabFilterType::Player("player".to_string())),
                    ])),
                },
            )]),
        };

        let config = serde_json::to_string(&config).unwrap();
        assert_eq!(
            config,
            r#"{"test2":{"persistent":true,"notify":true,"filters":{"and":[{"type":"channel","value":1},{"type":"player","value":"player"}]},"order":0}}"#
        );

        let config = ChatViewsConfig {
            views: HashMap::from([(
                "test3".to_string(),
                ChatTabConfig {
                    name: "test3".to_string(),
                    order: 0,
                    options: ChatTabOptions {
                        persistent: true,
                        notify: true,
                    },
                    filters: Some(ChatTabFilterTree::Or(vec![
                        ChatTabFilterTree::Filter(ChatTabFilterType::Channel(1)),
                        ChatTabFilterTree::And(vec![
                            ChatTabFilterTree::Filter(ChatTabFilterType::Player(
                                "player".to_string(),
                            )),
                            ChatTabFilterTree::Filter(ChatTabFilterType::Word("word".to_string())),
                        ]),
                    ])),
                },
            )]),
        };

        // println!("{}", serde_json::to_string_pretty(&config).unwrap());
        let config = serde_json::to_string(&config).unwrap();
        assert_eq!(
            config,
            r#"{"test3":{"persistent":true,"notify":true,"filters":{"or":[{"type":"channel","value":1},{"and":[{"type":"player","value":"player"},{"type":"word","value":"word"}]}]},"order":0}}"#
        );
    }
}
