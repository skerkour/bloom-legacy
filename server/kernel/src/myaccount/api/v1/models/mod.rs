use serde::{Serialize, Deserialize};
use crate::accounts::domain::session;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartRegistrationBody {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartRegistrationResponse {
    pub id: uuid::Uuid,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerifyPendingAccountBody {
    pub id: uuid::Uuid,
    pub code: String,
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteRegistrationBody {
    pub id:  uuid::Uuid,
    pub username: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteRegistrationResponse {
    pub id: uuid::Uuid,
    pub token: String,
}

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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateEmailBody {
    pub email: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateEmailResponse {
    pub id: uuid::Uuid, // id of the new pending_email
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct VerifyEmailBody {
    pub id: uuid::Uuid,
    pub code: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResetPassowrdBody {
    pub id: uuid::Uuid,
    pub token: String,
    pub new_password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PasswordResetRequestBody {
    pub email_or_username: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NewCodeBody {
    pub id: uuid::Uuid,
}
