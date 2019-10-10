use super::{from_message, Message};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationStart {
    pub display_name: String,
    pub email: String,
}
from_message!(RegistrationStart, Message::AuthRegistrationStart);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationStarted {
    pub id: uuid::Uuid,
}
from_message!(RegistrationStarted, Message::AuthRegistrationStarted);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationVerify {
    pub id: uuid::Uuid,
    pub code: String,
}
from_message!(RegistrationVerify, Message::AuthRegistrationVerify);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationComplete {
    pub id: uuid::Uuid,
    pub username: String,
    pub auth_key: String,
}
from_message!(RegistrationComplete, Message::AuthRegistrationComplete);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationSendNewCode {
    pub id: uuid::Uuid,
}
from_message!(RegistrationSendNewCode, Message::AuthRegistrationNewCode);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Session {
    pub id: uuid::Uuid,
    pub token: String,
}
from_message!(Session, Message::AuthSessionStarted);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignIn {
    pub username: String,
    pub auth_key: String,
}
from_message!(SignIn, Message::AuthSignIn);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignOut {}
from_message!(SignOut, Message::AuthSignOut);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RevokeSesison {
    pub id: uuid::Uuid,
}
from_message!(RevokeSesison, Message::AuthRevokeSession);

////////////////////////////////////////////////////////////////////////////////////////////////////
/// GUI
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiRegistrationComplete {
    pub id: uuid::Uuid,
    pub username: String,
    pub password: String,
}
from_message!(
    GuiRegistrationComplete,
    Message::AuthGuiRegistrationComplete
);

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct GuiSignIn {
    pub username: String,
    pub password: String,
}
from_message!(GuiSignIn, Message::AuthGuiSignIn);
