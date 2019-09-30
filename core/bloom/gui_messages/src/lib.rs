use serde::{Deserialize, Serialize};

pub mod auth;
pub mod to_remove;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Message {
    #[serde(rename = "gui.auth.registration_start")]
    AuthRegistrationStart(auth::RegistrationStart),
    #[serde(rename = "gui.auth.registration_started")]
    AuthRegistrationStarted(auth::RegistrationStarted),

    #[serde(rename = "gui.to_remove.tick")]
    ToRemoveTick(to_remove::Tick),
}
