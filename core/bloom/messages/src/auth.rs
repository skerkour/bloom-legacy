use super::Message;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartRegistration {
    pub display_name: String,
    pub email: String,
    pub auth_key: String,
}

impl From<StartRegistration> for Message {
    fn from(data: StartRegistration) -> Self {
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationVerify {
    pub id: uuid::Uuid,
    pub code: String,
}

impl From<RegistrationVerify> for Message {
    fn from(data: RegistrationVerify) -> Self {
        Message::AuthRegistrationVerify(data)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationComplete {
    pub id: uuid::Uuid,
    pub username: String,
}

impl From<RegistrationComplete> for Message {
    fn from(data: RegistrationComplete) -> Self {
        Message::AuthRegistrationComplete(data)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationSendNewCode {
    pub id: uuid::Uuid,
}

impl From<RegistrationSendNewCode> for Message {
    fn from(data: RegistrationSendNewCode) -> Self {
        Message::AuthRegistrationNewCode(data)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Session {
    pub id: uuid::Uuid,
    pub token: String,
}

impl From<Session> for Message {
    fn from(data: Session) -> Self {
        Message::AuthSessionStarted(data)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignIn {
    pub username: String,
    pub auth_key: String,
}

impl From<SignIn> for Message {
    fn from(data: SignIn) -> Self {
        Message::AuthSignIn(data)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RevokeSesison {
    pub id: uuid::Uuid,
}

impl From<RevokeSesison> for Message {
    fn from(data: RevokeSesison) -> Self {
        Message::AuthRevokeSession(data)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
/// GUI
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiRegistrationStart {
    pub display_name: String,
    pub email: String,
    pub password: String,
}

impl From<GuiRegistrationStart> for Message {
    fn from(data: GuiRegistrationStart) -> Self {
        Message::AuthGuiRegistrationStart(data)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiSignIn {
    pub username: String,
    pub password: String,
}

impl From<GuiSignIn> for Message {
    fn from(data: GuiSignIn) -> Self {
        Message::AuthGuiSignIn(data)
    }
}
