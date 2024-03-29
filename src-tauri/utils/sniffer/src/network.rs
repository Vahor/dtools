use std::collections::HashMap;

use anyhow::Result as AnyResult;
use pcap::Capture;
use std::sync::{Arc, Mutex};
use tauri::AppHandle;

use crate::parser::metadata::PacketMetadata;
use crate::parser::wrapper::DataWrapper;
use crate::protocol::{EventId, ProtocolEvent};

pub type Listener = fn(&ProtocolEvent);
pub type ListenerId = String;
pub type Subscription = (ListenerId, Listener);

#[derive(Debug)]
pub struct PacketListener {
    subscriptions: Arc<Mutex<HashMap<EventId, Vec<Subscription>>>>,
}

impl PacketListener {
    pub fn new() -> PacketListener {
        return PacketListener {
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
        };
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

    pub fn notify(&self, event: &ProtocolEvent) {
        PacketListener::_notify(&self.subscriptions.lock().unwrap(), event);
    }

    fn _notify(subscriptions: &HashMap<EventId, Vec<Subscription>>, event: &ProtocolEvent) {
        let listeners = subscriptions.get(&event.id.unwrap());
        if let Some(listeners) = listeners {
            for (_, listener) in listeners {
                listener(event);
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

    pub fn run(&self, handle: &AppHandle) -> AnyResult<()> {
        // TODO: configure device
        let mut cap = Capture::from_device("en0")
            .unwrap()
            .immediate_mode(true)
            .open()
            .expect("Failed to open device");
        cap.direction(pcap::Direction::In).unwrap();

        // TODO: configure port
        cap.filter("tcp port 5555", false).unwrap();

        let handle = handle.clone(); // TODO: check clone

        let subscriptions = self.subscriptions.clone();

        tauri::async_runtime::spawn(async move {
            println!("Sniffer started");
            let mut previous_frame_buffer_data: Vec<u8> = Vec::new();
            while let Ok(packet) = cap.next_packet() {
                let data = packet.data.to_vec();
                previous_frame_buffer_data.extend_from_slice(&data);

                let final_data = previous_frame_buffer_data.clone();
                let current_frame_buffer = &mut DataWrapper::new(final_data);
                let metadata = PacketMetadata::from_buffer(current_frame_buffer);

                match metadata {
                    Err(err) => match err {
                        crate::parser::metadata::ParseResult::Invalid => {
                            previous_frame_buffer_data.clear();
                        }
                        _ => {}
                    },
                    Ok(metadata) => {
                        previous_frame_buffer_data.clear();

                        // whitelist
                        if PacketListener::_has_subscriptions(
                            &subscriptions.lock().unwrap(),
                            &metadata.id,
                        ) {
                            dbg!("ok", &metadata.id, &handle);
                        } else {
                            dbg!(&metadata.id, &handle);
                            continue;
                        }
                    }
                };
            }
        });

        return Ok(());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pcap_file::pcap::PcapReader;
    use std::fs::File;

    #[test]
    fn test_packet_listener() {
        let mut listener = PacketListener {
            subscriptions: Arc::new(Mutex::new(HashMap::new())),
        };

        assert_eq!(listener.subscriptions.lock().unwrap().len(), 0);

        let listener_id = "test".to_string();
        let listener_fn = |_event: &ProtocolEvent| {};
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

    #[test]
    fn test_with_capture() {
        let capture = File::open("tests/fixtures/cap.pcap").unwrap();
        let mut reader = PcapReader::new(capture).unwrap();

        // TODO: find a way to avoid copy/pasting the code

        let mut previous_frame_buffer_data: Vec<u8> = Vec::new();
        while let Some(packet) = reader.next_packet() {
            let data = packet.unwrap().data.to_vec();
            previous_frame_buffer_data.extend_from_slice(&data);

            let final_data = previous_frame_buffer_data.clone();
            let current_frame_buffer = &mut DataWrapper::new(final_data);
            let metadata = PacketMetadata::from_buffer(current_frame_buffer);

            match metadata {
                Err(err) => match err {
                    crate::parser::metadata::ParseResult::Invalid => {
                        previous_frame_buffer_data.clear();
                    }
                    _ => {}
                },
                Ok(metadata) => {
                    previous_frame_buffer_data.clear();

                    // whitelist
                    if metadata.id != 64 {
                        continue;
                    }

                    println!("{:?}", metadata.id);
                }
            };
        }
    }
}
