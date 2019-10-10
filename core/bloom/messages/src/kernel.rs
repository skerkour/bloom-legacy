use super::{from_message, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Empty {}
from_message!(Empty, Message::Empty);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HelloWorld {
    pub hello: String,
}
from_message!(HelloWorld, Message::HelloWorld);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    pub code: String,
    pub message: String,
}
from_message!(Error, Message::Error);
