use serde::{Deserialize, Serialize};
use specta::Type;

use crate::features::windows::WindowOptions;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatConfig {
    pub views: Vec<ChatTabConfig>,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ChatTabConfig {
    #[serde(flatten)]
    pub options: ChatTabOptions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<ChatTabFilterTree>,
    pub window: Option<WindowOptions>,
}

#[derive(Clone, Serialize, Deserialize, Debug, Type)]
pub struct ChatTabOptions {
    pub persistent: bool,
    pub notify: bool,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ChatTabFilterTree {
    And(Vec<ChatTabFilterTree>),
    Or(Vec<ChatTabFilterTree>),

    #[serde(untagged)]
    Filter(ChatTabFilterType),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
#[serde(tag = "type", content = "value")]
#[serde(rename_all = "camelCase")]
pub enum ChatTabFilterType {
    Channel(u8),
    Player(String),
    Word(String),
}

impl Default for ChatConfig {
    fn default() -> Self {
        ChatConfig { views: Vec::new() }
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

impl Default for ChatTabConfig {
    fn default() -> Self {
        ChatTabConfig {
            options: ChatTabOptions::default(),
            filters: None,
            window: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let config = ChatConfig::default();
        assert_eq!(config.views.len(), 0);
    }

    #[test]
    fn test_deserialize() {
        let config = r#"{"tabs":[]}"#;
        let config: ChatConfig = serde_json::from_str(config).unwrap();
        assert_eq!(config.views.len(), 0);
    }

    #[test]
    fn test_serialize() {
        let config = ChatConfig::default();
        let config = serde_json::to_string(&config).unwrap();
        assert_eq!(config, r#"{"tabs":[]}"#);

        let config = ChatConfig {
            views: vec![ChatTabConfig {
                window: None,
                options: ChatTabOptions {
                    persistent: true,
                    notify: true,
                },
                filters: Some(ChatTabFilterTree::Filter(ChatTabFilterType::Channel(1))),
            }],
        };

        let config = serde_json::to_string(&config).unwrap();
        assert_eq!(
            config,
            r#"{"tabs":[{"persistent":true,"notify":true,"filters":{"type":"channel","value":1}}]}"#
        );

        let config = ChatConfig {
            views: vec![ChatTabConfig {
                window: None,
                options: ChatTabOptions {
                    persistent: true,
                    notify: true,
                },
                filters: Some(ChatTabFilterTree::And(vec![
                    ChatTabFilterTree::Filter(ChatTabFilterType::Channel(1)),
                    ChatTabFilterTree::Filter(ChatTabFilterType::Player("player".to_string())),
                ])),
            }],
        };

        let config = serde_json::to_string(&config).unwrap();
        assert_eq!(
            config,
            r#"{"tabs":[{"persistent":true,"notify":true,"filters":{"and":[{"type":"channel","value":1},{"type":"player","value":"player"}]}}]}"#
        );

        let config = ChatConfig {
            views: vec![ChatTabConfig {
                window: None,
                options: ChatTabOptions {
                    persistent: true,
                    notify: true,
                },
                filters: Some(ChatTabFilterTree::Or(vec![
                    ChatTabFilterTree::Filter(ChatTabFilterType::Channel(1)),
                    ChatTabFilterTree::And(vec![
                        ChatTabFilterTree::Filter(ChatTabFilterType::Player("player".to_string())),
                        ChatTabFilterTree::Filter(ChatTabFilterType::Word("word".to_string())),
                    ]),
                ])),
            }],
        };

        // println!("{}", serde_json::to_string_pretty(&config).unwrap());
        let config = serde_json::to_string(&config).unwrap();
        assert_eq!(
            config,
            r#"{"tabs":[{"persistent":true,"notify":true,"filters":{"or":[{"type":"channel","value":1},{"and":[{"type":"player","value":"player"},{"type":"word","value":"word"}]}]}}]}"#
        );
    }
}
