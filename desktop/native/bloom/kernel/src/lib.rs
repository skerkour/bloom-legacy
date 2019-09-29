// use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, RecvTimeoutError};
// use std::thread;
use rand::prelude::*;
use serde::{Deserialize, Serialize};
use std::time;

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageIn {
    pub id: Option<String>,
    pub data: MessageData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageOut {
    pub id: Option<String>,
    pub data: Option<MessageData>,
    pub error: Option<String>,
}

impl From<MessageIn> for MessageOut {
    fn from(message: MessageIn) -> Self {
        return MessageOut {
            id: message.id,
            data: Some(message.data),
            error: None,
        };
    }
}

/// Represents the data that will be received by the `poll` method. It may
/// include different types of data or be replaced with a more simple type,
/// e.g., `Vec<u8>`.
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "data")]
pub enum MessageData {
    Tick { count: u64 },
}

pub struct App {
    counter: u64,
    gui_sender: mpsc::Sender<MessageOut>,
    gui_receiver: mpsc::Receiver<MessageIn>,
}

impl App {
    pub fn new(
        gui_sender: mpsc::Sender<MessageOut>,
        gui_receiver: mpsc::Receiver<MessageIn>,
    ) -> App {
        return App {
            counter: 0,
            gui_sender,
            gui_receiver,
        };
    }

    pub fn run(&mut self) {
        loop {
            // thread::sleep(time::Duration::from_secs(1));
            // self.gui_sender.send(format!("hello from eventloop_thread {}", self.counter));
            // self.gui_sender.send(Event::Tick { count: self.counter }).expect("Send failed");
            // self.counter += 1;
            match self.gui_receiver.recv_timeout(time::Duration::from_secs(1)) {
                // use select instead
                Ok(event) => self.handle_js_event(event),
                Err(RecvTimeoutError::Timeout) => {
                    let mut rng = rand::thread_rng();
                    let n: f64 = rng.gen();
                    if n > 0.5 {
                        self.gui_sender
                            .send(MessageOut {
                                id: None,
                                data: Some(MessageData::Tick {
                                    count: self.counter,
                                }),
                                error: None,
                            })
                            .expect("Send failed");
                    }
                    self.counter += 1;
                }
                Err(RecvTimeoutError::Disconnected) => panic!("Failed to receive event"),
            }
        }
    }

    fn handle_js_event(&self, event: MessageIn) {
        self.gui_sender.send(event.into()).expect("Send failed"); // send back event
    }
}
