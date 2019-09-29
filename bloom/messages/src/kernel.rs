use super::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NoData {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HelloWorld {
    pub hello: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    pub code: String,
    pub message: String,
}

impl From<NoData> for Message {
    fn from(data: NoData) -> Self {
        Message::KernelNoData(data)
    }
}

impl From<HelloWorld> for Message {
    fn from(data: HelloWorld) -> Self {
        Message::KernelHelloWorld(data)
    }
}

impl From<Error> for Message {
    fn from(data: Error) -> Self {
        Message::KernelError(data)
    }
}
