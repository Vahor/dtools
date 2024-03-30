use std::collections::HashMap;

use core::fmt::Debug;
use pcap::{Activated, Capture};
use std::sync::{Arc, Mutex};
use thiserror::Error;
use tracing::{info, warn};

use crate::{
    node::Node,
    sniffer::parser::{
        metadata::{PacketMetadata, ParseResult},
        packet::PacketParser,
        wrapper::DataWrapper,
    },
};

use super::{parser::packet::Packet, protocol::EventId};

pub type Listener = fn(&Packet, &Node);
pub type ListenerId = String;
pub type Subscription = (ListenerId, Listener);

#[derive(Debug)]
pub struct PacketListener {
    subscriptions: Arc<Mutex<HashMap<EventId, Vec<Subscription>>>>,
    node: Option<Arc<Node>>,
}

impl PacketListener {
    pub fn new() -> PacketListener {
        return PacketListener {
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
            node: None,
        };
    }

    pub fn set_node(&mut self, node: Arc<Node>) {
        self.node = Some(node);
    }

    pub fn subscribe(&mut self, event: EventId, listener_id: ListenerId, listener: Listener) {
        self.subscriptions
            .lock()
            .unwrap()
            .entry(event)
            .or_default()
            .push((listener_id, listener));
    }

    pub fn unsubscribe(&mut self, event: &EventId, listener_id: ListenerId) {
        self.subscriptions
            .lock()
            .unwrap()
            .get_mut(event)
            .map(|listeners| listeners.retain(|(id, _)| id != &listener_id));
    }

    pub fn notify(&self, event: &Packet) {
        PacketListener::_notify(
            &self.subscriptions.lock().unwrap(),
            event,
            &self.node.as_ref().unwrap(),
        );
    }

    fn _notify(subscriptions: &HashMap<EventId, Vec<Subscription>>, packet: &Packet, node: &Node) {
        let listeners = subscriptions.get(&packet.id);
        if let Some(listeners) = listeners {
            for (_, listener) in listeners {
                listener(packet, node);
            }
        }
    }

    pub fn has_subscriptions(&self, event: &EventId) -> bool {
        return PacketListener::_has_subscriptions(&self.subscriptions.lock().unwrap(), event);
    }

    fn _has_subscriptions(
        subscriptions: &HashMap<EventId, Vec<Subscription>>,
        event: &EventId,
    ) -> bool {
        return subscriptions
            .get(event)
            .map_or(false, |listeners| !listeners.is_empty());
    }

    pub fn run(&self) -> Result<(), PacketListenerError> {
        if self.node.is_none() {
            return Err(PacketListenerError::InvalidCaptureDevice);
        }

        let config = self.node.as_ref().unwrap().config.config.read().unwrap();
        let interface = config.network.interface.as_str();
        let port = config.network.port;

        info!(
            "Starting sniffer on interface: {} and port: {}",
            interface, port
        );

        let mut cap = Capture::from_device(interface)
            .unwrap()
            .immediate_mode(true)
            .open()
            .expect("Failed to open device");
        cap.direction(pcap::Direction::In).unwrap();

        cap.filter(format!("tcp port {}", port).as_str(), false)
            .unwrap();

        self.run_with_capture(cap.into())
    }

    pub fn run_with_capture(
        &self,
        mut cap: Capture<dyn Activated>,
    ) -> Result<(), PacketListenerError> {
        if self.node.is_none() {
            return Err(PacketListenerError::InvalidCaptureDevice);
        }

        let subscriptions = self.subscriptions.clone();
        let procol_manager = self.node.as_ref().unwrap().protocol.clone();
        let node = self.node.clone().unwrap();

        tauri::async_runtime::spawn(async move {
            let mut previous_frame_buffer_data: Vec<u8> = Vec::new();
            while let Ok(packet) = cap.next_packet() {
                let data = packet.data.to_vec();
                previous_frame_buffer_data.extend_from_slice(&data);

                let final_data = previous_frame_buffer_data.clone();
                let current_frame_buffer = &mut DataWrapper::new(final_data);
                let metadata = PacketMetadata::from_buffer(current_frame_buffer);

                match metadata {
                    Err(err) => match err {
                        ParseResult::Invalid => {
                            previous_frame_buffer_data.clear();
                        }
                        _ => {}
                    },
                    Ok(metadata) => {
                        previous_frame_buffer_data.clear();

                        // TODO: check 64, 110, 16203 ids

                        // whitelist
                        if PacketListener::_has_subscriptions(
                            &subscriptions.lock().unwrap(),
                            &metadata.id,
                        ) {
                            let mut parser = PacketParser::from_metadata(&metadata);
                            match parser.parse(&procol_manager) {
                                Ok(packet) => {
                                    info!("Packet: {:?}", packet);
                                    PacketListener::_notify(
                                        &subscriptions.lock().unwrap(),
                                        &packet,
                                        &node,
                                    );
                                }
                                Err(err) => {
                                    warn!("Failed to parse packet: {:?}", err);
                                }
                            }
                        }
                    }
                };
            }
        });

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum PacketListenerError {
    #[error("Failed to open device")]
    FailedToOpenDevice,
    #[error("Invalid capture device")]
    InvalidCaptureDevice,
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test_packet_listener() {
        let mut listener = PacketListener::new();

        assert_eq!(listener.subscriptions.lock().unwrap().len(), 0);

        let listener_id = "test".to_string();
        let listener_fn = |_event: &Packet, _: &Node| {};
        let event = 0;

        listener.subscribe(event.clone(), listener_id.clone(), listener_fn);
        assert_eq!(listener.subscriptions.lock().unwrap().len(), 1);
        assert_eq!(
            listener
                .subscriptions
                .lock()
                .unwrap()
                .get(&event)
                .unwrap()
                .len(),
            1
        );

        listener.unsubscribe(&event, listener_id);
        assert_eq!(listener.subscriptions.lock().unwrap().len(), 1);
        assert_eq!(
            listener
                .subscriptions
                .lock()
                .unwrap()
                .get(&event)
                .unwrap()
                .len(),
            0
        );
    }

    #[tokio::test]
    async fn test_with_capture() {
        let cap = Capture::from_file("tests/fixtures/cap.pcap").unwrap();
        let path = "tests/fixtures/".to_string();
        let path = Path::new(&path);
        let node = Node::new(path, None, false).await;
        if let Err(err) = node {
            panic!("Failed to create node: {:?}", err);
        }
        let node = node.unwrap();
        let listener_fn = |event: &Packet, node: &Node| {
            let key = event.id.to_string();
            let mut store = node.store.lock().unwrap();
            match store.get(&key) {
                Some(count) => {
                    let count = count.parse::<u32>().unwrap();
                    let count = count + 1;
                    let count = count.to_string();
                    store.insert(key, count);
                }
                None => {
                    store.insert(key, "1".to_string());
                }
            }
        };

        let mut listener = node.packet_listener.lock().unwrap();
        let id = "test".to_string();
        // listener.subscribe(213, id.clone(), listener_fn);
        listener.subscribe(4879, id.clone(), listener_fn);

        let res = listener.run_with_capture(cap.into());
        if let Err(err) = res {
            panic!("Failed to run with capture: {:?}", err);
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        info!("Store: {:?}", node.store.lock().unwrap());
    }
}
