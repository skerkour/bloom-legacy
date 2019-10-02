use super::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Empty {}

impl From<Empty> for Message {
    fn from(data: Empty) -> Self {
        Message::Empty(data)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HelloWorld {
    pub hello: String,
}

impl From<HelloWorld> for Message {
    fn from(data: HelloWorld) -> Self {
        Message::HelloWorld(data)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    pub code: String,
    pub message: String,
}

impl From<Error> for Message {
    fn from(data: Error) -> Self {
        Message::Error(data)
    }
}
