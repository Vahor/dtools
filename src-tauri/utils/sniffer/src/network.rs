use std::collections::HashMap;

use anyhow::Result as AnyResult;
use core::fmt::Debug;
use pcap::{Activated, Capture};
use std::sync::{Arc, Mutex};
use tauri::AppHandle;

use crate::parser::metadata::PacketMetadata;
use crate::parser::packet::{Packet, PacketParser};
use crate::parser::wrapper::DataWrapper;
use crate::protocol::EventId;

pub type Listener = fn(&Packet, &AppHandle);
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

    pub fn notify(&self, event: &Packet, handle: &AppHandle) {
        PacketListener::_notify(&self.subscriptions.lock().unwrap(), event, handle);
    }

    fn _notify(
        subscriptions: &HashMap<EventId, Vec<Subscription>>,
        packet: &Packet,
        handle: &AppHandle,
    ) {
        let listeners = subscriptions.get(&packet.id);
        if let Some(listeners) = listeners {
            for (_, listener) in listeners {
                listener(packet, handle);
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

        return self.run_with_capture(cap.into(), handle);
    }

    pub fn run_with_capture(
        &self,
        mut cap: Capture<dyn Activated>,
        handle: &AppHandle,
    ) -> AnyResult<()> {
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
                            let mut parser = PacketParser::from_metadata(&metadata);
                            if let Some(packet) = parser.parse(&handle) {
                                PacketListener::_notify(
                                    &subscriptions.lock().unwrap(),
                                    &packet,
                                    &handle,
                                );
                                dbg!("ok", &packet);
                            }
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
    use tauri::Manager;

    use super::*;

    #[test]
    fn test_packet_listener() {
        let mut listener = PacketListener::new();

        assert_eq!(listener.subscriptions.lock().unwrap().len(), 0);

        let listener_id = "test".to_string();
        let listener_fn = |_event: &Packet, _: &AppHandle| {};
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

    struct State {
        pub count: Arc<Mutex<i32>>,
    }

    #[test]
    fn test_with_capture() {
        let cap = Capture::from_file("tests/fixtures/cap.pcap").unwrap();
        // let mut listener = PacketListener::new();
        // let app = tauri::test::mock_app();
        // let handle: AppHandle<tauri::Wry> = app.handle();
        //
        // let state = State {
        //     count: Arc::new(Mutex::new(0)),
        // };
        //
        // app.manage(state);
        //
        // let listener_fn = |_event: &Packet, handle: &AppHandle| {
        //     let state = handle.state::<State>();
        //     let mut count = state.count.lock().unwrap();
        //     *count += 1;
        //     println!("count: {}", *count);
        // };
        //
        // listener.subscribe(64, "test".to_string(), listener_fn);
        //
        // listener.run_with_capture(cap.into(), &handle).unwrap();
    }
}
