// use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, RecvTimeoutError};
// use std::thread;
use serde::{Deserialize, Serialize};
use std::time;
use bloom_core::{messages, auth};

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
    gui_sender: mpsc::Sender<messages::Message>,
    gui_receiver: mpsc::Receiver<messages::Message>,
}

impl App {
    pub fn new(
        gui_sender: mpsc::Sender<messages::Message>,
        gui_receiver: mpsc::Receiver<messages::Message>,
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
                Ok(message) => self.handle_gui_message(message),
                Err(RecvTimeoutError::Timeout) => {
                    let n = crypto42::rand::uniform(100);
                    if n > 50 {
                        self.gui_sender
                            .send(messages::to_remove::Tick{
                                    count: self.counter,
                            }.into())
                            .expect("Send failed");
                    }
                    self.counter += 1;
                }
                Err(RecvTimeoutError::Disconnected) => panic!("Failed to receive message"),
            }
        }
    }

    fn handle_gui_message(&self, message: messages::Message) {
        match message {
            messages::Message::AuthGuiRegistrationStart(message) => {
                let res = auth::registration_start(message);
                self.gui_sender.send(res.into()).expect("Send failed"); // send response back
            },
            messages::Message::AuthRegistrationVerify(message) => {
                let res = auth::registration_verify(message);
                self.gui_sender.send(res.into()).expect("Send failed"); // send response back
            },
            // TODO: handle message not implemented
            _ => self.gui_sender.send(message.into()).expect("Send failed"), // send back message
        };
    }
}
