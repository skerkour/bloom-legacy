use serde::{Deserialize, Serialize};

pub mod auth;
pub mod kernel;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Message {
    #[serde(rename = "bloom.no_data")]
    KernelNoData(kernel::NoData),
    #[serde(rename = "bloom.hello_world")]
    KernelHelloWorld(kernel::HelloWorld),
    #[serde(rename = "bloom.error")]
    KernelError(kernel::Error),

    #[serde(rename = "auth.registration_start")]
    AuthRegistrationStart(auth::StartRegistration),
    #[serde(rename = "auth.registration_started")]
    AuthRegistrationStarted(auth::RegistrationStarted),
    #[serde(rename = "auth.registration_verify")]
    AuthRegistrationVerify(auth::RegistrationVerify),
    #[serde(rename = "auth.registration_complete")]
    AuthRegistrationComplete(auth::RegistrationComplete),
    #[serde(rename = "auth.registration_new_code")]
    AuthRegistrationNewCode(auth::RegistrationSendNewCode),
    #[serde(rename = "auth.session_started")]
    AuthSessionStarted(auth::Session),
}

impl From<kernel::NoData> for Message {
    fn from(data: kernel::NoData) -> Self {
        Message::KernelNoData(data)
    }
}

impl From<kernel::HelloWorld> for Message {
    fn from(data: kernel::HelloWorld) -> Self {
        Message::KernelHelloWorld(data)
    }
}

impl From<kernel::Error> for Message {
    fn from(data: kernel::Error) -> Self {
        Message::KernelError(data)
    }
}

impl From<auth::StartRegistration> for Message {
    fn from(data: auth::StartRegistration) -> Self {
        Message::AuthRegistrationStart(data)
    }
}

impl From<auth::RegistrationStarted> for Message {
    fn from(data: auth::RegistrationStarted) -> Self {
        Message::AuthRegistrationStarted(data)
    }
}

impl From<auth::RegistrationVerify> for Message {
    fn from(data: auth::RegistrationVerify) -> Self {
        Message::AuthRegistrationVerify(data)
    }
}

impl From<auth::RegistrationComplete> for Message {
    fn from(data: auth::RegistrationComplete) -> Self {
        Message::AuthRegistrationComplete(data)
    }
}

impl From<auth::Session> for Message {
    fn from(data: auth::Session) -> Self {
        Message::AuthSessionStarted(data)
    }
}

impl From<auth::RegistrationSendNewCode> for Message {
    fn from(data: auth::RegistrationSendNewCode) -> Self {
        Message::AuthRegistrationNewCode(data)
    }
}
