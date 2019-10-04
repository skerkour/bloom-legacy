use serde::{Deserialize, Serialize};

pub mod auth;
pub mod kernel;
pub mod to_remove;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", content = "data")]
pub enum Message {
    // kernel
    #[serde(rename = "empty")]
    Empty(kernel::Empty),
    #[serde(rename = "hello_world")]
    HelloWorld(kernel::HelloWorld),
    #[serde(rename = "error")]
    Error(kernel::Error),

    // auth
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
    #[serde(rename = "auth.sign_in")]
    AuthSignIn(auth::SignIn),
    #[serde(rename = "auth.sign_out")]
    AuthSignOut(kernel::Empty),
    #[serde(rename = "auth.revoke_session")]
    AuthRevokeSession(auth::RevokeSesison),
    // auth gui
    #[serde(rename = "auth.gui.registration_start")]
    AuthGuiRegistrationStart(auth::GuiRegistrationStart),
    #[serde(rename = "auth.gui.sign_in")]
    AuthGuiSignIn(auth::GuiSignIn),

    #[serde(rename = "gui.to_remove.tick")]
    ToRemoveTick(to_remove::Tick),
}
