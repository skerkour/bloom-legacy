use serde::{Deserialize, Serialize};

pub mod auth;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Message {
    #[serde(rename = "gui.auth.registration_start")]
    AuthRegistrationStart(auth::RegistrationStart),
}
