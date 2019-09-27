use serde::{Deserialize, Serialize};

pub mod auth;
pub mod kernel;


#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Message {
    #[serde(rename = "bloom.hello_world")]
    KernelHelloWorld(kernel::HelloWorld),

    #[serde(rename = "auth.start_registration")]
    AuthStartRegistration(auth::StartRegistration),
    #[serde(rename = "auth.registration_started")]
    AuthRegistrationStarted(auth::RegistrationStarted),
}

impl From<kernel::HelloWorld> for Message {
    fn from(data: kernel::HelloWorld) -> Self {
        Message::KernelHelloWorld(data)
    }
}

impl From<auth::StartRegistration> for Message {
    fn from(data: auth::StartRegistration) -> Self {
        Message::AuthStartRegistration(data)
    }
}

impl From<auth::RegistrationStarted> for Message {
    fn from(data: auth::RegistrationStarted) -> Self {
        Message::AuthRegistrationStarted(data)
    }
}
