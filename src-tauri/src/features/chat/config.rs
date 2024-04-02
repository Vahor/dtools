use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use specta::Type;

use crate::sniffer::parser::packet::Packet;

#[derive(Clone, Serialize, Deserialize, Debug, specta::Type)]
pub struct ChatViewsConfig {
    pub views: HashMap<String, ChatTabConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_tab_id: Option<String>,
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
#[serde(rename_all = "camelCase")]
pub struct ChatTabOptions {
    pub keep_history: bool,
    pub notify: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, specta::Type, tauri_specta::Event)]
pub struct ChatEvent {
    pub channel: u8,
    pub sender_name: String,
    pub content: String,
    pub timestamp: u32,
    pub objects: Option<Vec<HashMap<String, String>>>,
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
            objects: packet.data.get("objects").map(|objects| {
                objects
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|object| {
                        object
                            .as_object()
                            .unwrap()
                            .iter()
                            .map(|(key, value)| (key.to_string(), value.to_string()))
                            .collect()
                    })
                    .collect()
            }),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, specta::Type)]
#[serde(rename_all = "camelCase")]
pub enum ChatTabFilterTree {
    And(Vec<ChatTabFilterTree>),
    Or(Vec<ChatTabFilterTree>),

    // #[serde(untagged)] we ca't use untagged because of specta
    Leaf(ChatTabFilterType),
}

impl ChatTabFilterTree {
    pub fn evaluate(&self, data: &ChatEvent) -> bool {
        match self {
            ChatTabFilterTree::And(filters) => filters.iter().all(|filter| filter.evaluate(data)),
            ChatTabFilterTree::Or(filters) => filters.iter().any(|filter| filter.evaluate(data)),
            ChatTabFilterTree::Leaf(filter) => match filter {
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
            last_tab_id: None,
        }
    }
}

impl Default for ChatTabOptions {
    fn default() -> Self {
        ChatTabOptions {
            keep_history: true,
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
        let config = r#"{"views": {}}"#;
        let config: ChatViewsConfig = serde_json::from_str(config).unwrap();
        assert_eq!(config.views.len(), 0);

        let config = r#"{"views":{"id-test2":{"name":"test2","options":{"keepHistory":true,"notify":true},"filters":{"and":[{"type":"channel","value":1},{"type":"player","value":"player"}]},"order":0}}}"#;
        let config: ChatViewsConfig = serde_json::from_str(config).unwrap();
        assert_eq!(config.views.len(), 1);
        let tab = config.views.get("test").unwrap();
        assert_eq!(tab.name, "test");
        assert_eq!(tab.order, 0);
        assert_eq!(tab.options.keep_history, true);
        assert_eq!(tab.options.notify, true);
    }

    #[test]
    fn test_serialize() {
        let config = ChatViewsConfig::default();
        let config = serde_json::to_string(&config).unwrap();
        assert_eq!(config, r#"{"views":{}}"#);

        let config = ChatViewsConfig {
            last_tab_id: None,
            views: HashMap::from([(
                "id-test".to_string(),
                ChatTabConfig {
                    name: "test".to_string(),
                    order: 0,
                    options: ChatTabOptions {
                        keep_history: true,
                        notify: true,
                    },
                    filters: Some(ChatTabFilterTree::Leaf(ChatTabFilterType::Channel(1))),
                },
            )]),
        };

        let config = serde_json::to_string(&config).unwrap();
        assert_eq!(
            config,
            r#"{"views":{"id-test":{"name":"test","options":{"keepHistory":true,"notify":true},"filters":{"leaf":{"type":"channel","value":1}},"order":0}}}"#
        );

        let config = ChatViewsConfig {
            last_tab_id: None,
            views: HashMap::from([(
                "id-test2".to_string(),
                ChatTabConfig {
                    name: "test2".to_string(),
                    order: 0,
                    options: ChatTabOptions {
                        keep_history: true,
                        notify: true,
                    },
                    filters: Some(ChatTabFilterTree::And(vec![
                        ChatTabFilterTree::Leaf(ChatTabFilterType::Channel(1)),
                        ChatTabFilterTree::Leaf(ChatTabFilterType::Player("player".to_string())),
                    ])),
                },
            )]),
        };

        let config = serde_json::to_string(&config).unwrap();
        assert_eq!(
            config,
            r#"{"views":{"id-test2":{"name":"test2","options":{"keepHistory":true,"notify":true},"filters":{"and":[{"leaf":{"type":"channel","value":1}},{"leaf":{"type":"player","value":"player"}}]},"order":0}}}"#
        );

        let config = ChatViewsConfig {
            last_tab_id: None,
            views: HashMap::from([(
                "id-test3".to_string(),
                ChatTabConfig {
                    name: "test3".to_string(),
                    order: 0,
                    options: ChatTabOptions {
                        keep_history: true,
                        notify: true,
                    },
                    filters: Some(ChatTabFilterTree::Or(vec![
                        ChatTabFilterTree::Leaf(ChatTabFilterType::Channel(1)),
                        ChatTabFilterTree::And(vec![
                            ChatTabFilterTree::Leaf(ChatTabFilterType::Player(
                                "player".to_string(),
                            )),
                            ChatTabFilterTree::Leaf(ChatTabFilterType::Word("word".to_string())),
                        ]),
                    ])),
                },
            )]),
        };

        // println!("{}", serde_json::to_string_pretty(&config).unwrap());
        let config = serde_json::to_string(&config).unwrap();
        assert_eq!(
            config,
            r#"{"views":{"id-test3":{"name":"test3","options":{"keepHistory":true,"notify":true},"filters":{"or":[{"leaf":{"type":"channel","value":1}},{"and":[{"leaf":{"type":"player","value":"player"}},{"leaf":{"type":"word","value":"word"}}]}]},"order":0}}}"#
        );
    }
}
