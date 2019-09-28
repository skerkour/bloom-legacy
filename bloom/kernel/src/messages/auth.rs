use super::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartRegistration {
    pub display_name: String,
    pub email: String,
    pub auth_key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationStarted {
    pub id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationVerify {
    pub id: uuid::Uuid,
    pub code: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationComplete {
    pub id: uuid::Uuid,
    pub username: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationSendNewCode {
    pub id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Session {
    pub id: uuid::Uuid,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignIn {
    pub username: String,
    pub auth_key: String,
}

impl From<StartRegistration> for Message {
    fn from(data: StartRegistration) -> Self {
        Message::AuthRegistrationStart(data)
    }
}

impl From<RegistrationStarted> for Message {
    fn from(data: RegistrationStarted) -> Self {
        Message::AuthRegistrationStarted(data)
    }
}

impl From<RegistrationVerify> for Message {
    fn from(data: RegistrationVerify) -> Self {
        Message::AuthRegistrationVerify(data)
    }
}

impl From<RegistrationComplete> for Message {
    fn from(data: RegistrationComplete) -> Self {
        Message::AuthRegistrationComplete(data)
    }
}

impl From<Session> for Message {
    fn from(data: Session) -> Self {
        Message::AuthSessionStarted(data)
    }
}

impl From<RegistrationSendNewCode> for Message {
    fn from(data: RegistrationSendNewCode) -> Self {
        Message::AuthRegistrationNewCode(data)
    }
}
