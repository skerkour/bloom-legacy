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
    #[serde(rename = "auth.sign_in")]
    AuthSignIn(auth::SignIn),
    #[serde(rename = "auth.sign_out")]
    AuthSignOut(kernel::NoData),
    #[serde(rename = "auth.revoke_session")]
    AuthRevokeSession(auth::RevokeSesison),
}
