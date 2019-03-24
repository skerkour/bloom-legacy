use serde::{Serialize, Deserialize};
use crate::services::account::domain::{
    session,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterBody {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RegisterResponse {
    pub id: uuid::Uuid,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerifyBody {
    pub id: String,
    pub code: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteRegistrationBody {
    pub id: String,
    pub code: String,
    pub username: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteRegistrationResponse {
    pub id: uuid::Uuid,
    pub token: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NoData {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignInBody {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SignInResponse {
    pub id: uuid::Uuid,
    pub token: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MeResponse {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub avatar_url: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Session {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub ip: String,
    pub location: Option<session::Location>,
    pub device: session::Device,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateAccount {
    pub avatar_url: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdatePassword {
    pub current_password: String,
    pub new_password: String,
}
