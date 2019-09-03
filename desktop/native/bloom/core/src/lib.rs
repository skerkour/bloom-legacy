use std::sync::{Arc, Mutex};
use std::sync::mpsc::{self, RecvTimeoutError};
use std::thread;
use std::time;
use serde::{Serialize, Deserialize};




#[derive(Serialize, Deserialize, Debug)]
pub struct JsEvent {
    pub id: Option<String>,
    pub data: Event,
    pub error: Option<String>
}


/// Represents the data that will be received by the `poll` method. It may
/// include different types of data or be replaced with a more simple type,
/// e.g., `Vec<u8>`.
#[derive(Serialize, Deserialize, Debug)]
pub enum Event {
    Tick { count: u64 },
}

pub struct App {
    counter: u64,
    gui_sender: mpsc::Sender<JsEvent>,
    gui_receiver: mpsc::Receiver<JsEvent>,
}

impl App {
    pub fn new(gui_sender: mpsc::Sender<JsEvent>, gui_receiver: mpsc::Receiver<JsEvent>) -> App {
        return App{
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
            match self.gui_receiver.recv_timeout(time::Duration::from_secs(1)) { // use select instead
                Ok(event) => {
                    self.gui_sender.send(event).expect("Send failed"); // send back event
                },
                Err(RecvTimeoutError::Timeout) => {
                    self.gui_sender.send(JsEvent{
                        id: None,
                        data: Event::Tick { count: self.counter },
                        error: None,
                    }).expect("Send failed");
                    self.counter += 1;
                },
                Err(RecvTimeoutError::Disconnected) => panic!("Failed to receive event"),
            }
        }
    }
}
