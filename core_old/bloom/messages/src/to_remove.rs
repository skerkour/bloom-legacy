use super::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tick {
    pub count: u64,
}

impl From<Tick> for Message {
    fn from(data: Tick) -> Self {
        Message::ToRemoveTick(data)
    }
}
