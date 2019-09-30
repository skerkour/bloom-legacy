use serde::{Deserialize, Serialize};

pub mod auth;
pub mod to_remove;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Message {
    #[serde(rename = "gui.no_data")]
    NoData(NoData),
    #[serde(rename = "gui.error")]
    Error(Error),

    #[serde(rename = "gui.auth.registration_start")]
    AuthRegistrationStart(auth::RegistrationStart),
    #[serde(rename = "gui.auth.registration_started")]
    AuthRegistrationStarted(auth::RegistrationStarted),
    #[serde(rename = "gui.auth.registration_verify")]
    AuthRegistrationVerify(auth::RegistrationVerify),

    #[serde(rename = "gui.to_remove.tick")]
    ToRemoveTick(to_remove::Tick),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NoData {}

impl From<NoData> for Message {
    fn from(data: NoData) -> Self {
        Message::NoData(data)
    }
}

impl From<api_messages::kernel::NoData> for NoData {
    fn from(_: api_messages::kernel::NoData) -> Self {
        NoData {}
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
