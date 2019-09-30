use super::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationStart {
    pub display_name: String,
    pub email: String,
    pub password: String,
}

impl From<RegistrationStart> for Message {
    fn from(data: RegistrationStart) -> Self {
        Message::AuthRegistrationStart(data)
    }
}
