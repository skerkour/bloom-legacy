// use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, RecvTimeoutError};
// use std::thread;
use bloom_core::messages;
use serde::{Deserialize, Serialize};
use std::time;

// #[derive(Serialize, Deserialize, Debug)]
// pub struct MessageIn {
//     pub id: Option<String>,
//     pub data: MessageData,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct MessageOut {
//     pub id: Option<String>,
//     pub data: Option<MessageData>,
//     pub error: Option<String>,
// }

// impl From<MessageIn> for MessageOut {
//     fn from(message: MessageIn) -> Self {
//         return MessageOut {
//             id: message.id,
//             data: Some(message.data),
//             error: None,
//         };
//     }
// }

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
    gui_sender: mpsc::Sender<bloom_core::NativeMessage>,
    gui_receiver: mpsc::Receiver<bloom_core::NativeMessage>,
}

impl App {
    pub fn new(
        gui_sender: mpsc::Sender<bloom_core::NativeMessage>,
        gui_receiver: mpsc::Receiver<bloom_core::NativeMessage>,
    ) -> App {
        return App {
            counter: 0,
            gui_sender,
            gui_receiver,
        };
    }

    pub fn run(&mut self) {
        loop {
            match self.gui_receiver.recv_timeout(time::Duration::from_secs(1)) {
                // use select instead
                Ok(message) => {
                    let id = message.id;
                    let message = bloom_core::handle_message(message.message);
                    self.gui_sender
                        .send(bloom_core::NativeMessage { id, message })
                        .expect("error sending message back");
                }
                Err(RecvTimeoutError::Timeout) => {
                    let n = crypto42::rand::uniform(100);
                    if n > 50 {
                        let message = messages::to_remove::Tick {
                            count: self.counter,
                        }
                        .into();
                        self.gui_sender
                            .send(bloom_core::NativeMessage { id: None, message })
                            .expect("Send failed");
                    }
                    self.counter += 1;
                }
                Err(RecvTimeoutError::Disconnected) => panic!("Failed to receive message"),
            }
        }
    }
}
