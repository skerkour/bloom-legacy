use serde::{Deserialize, Serialize};

pub mod auth;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Message {
    #[serde(rename = "auth.start_registration")]
    AuthStartRegistration(auth::StartRegistration),
    #[serde(rename = "auth.registration_started")]
    AuthRegistrationStarted(auth::RegistrationStarted),
}
