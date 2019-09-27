use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartRegistration {
    pub display_name: String,
    pub email: String,
    pub auth_key: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegistrationStarted {
    pub id: uuid::Uuid,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerifyPendingAccount {
    pub id: uuid::Uuid,
    pub code: String,
}
