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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationStarted {
    pub id: uuid::Uuid,
}

impl From<RegistrationStarted> for Message {
    fn from(data: RegistrationStarted) -> Self {
        Message::AuthRegistrationStarted(data)
    }
}

impl From<api_messages::auth::RegistrationStarted> for RegistrationStarted {
    fn from(data: api_messages::auth::RegistrationStarted) -> Self {
        RegistrationStarted { id: data.id }
    }
}
