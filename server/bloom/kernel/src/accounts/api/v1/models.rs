use crate::accounts::domain::{account, session};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartRegistrationBody {
    pub full_name: String,
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
    pub id: uuid::Uuid,
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
    pub is_admin: bool,
    pub bio: String,
    pub display_name: String,
}

impl From<account::Account> for MeResponse {
    fn from(account: account::Account) -> Self {
        MeResponse {
            id: account.id,
            created_at: account.created_at,
            first_name: account.first_name,
            last_name: account.last_name,
            username: account.username,
            email: account.email,
            avatar_url: account.avatar_url,
            is_admin: account.is_admin,
            bio: account.bio,
            display_name: account.display_name,
        }
    }
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
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub bio: Option<String>,
    pub display_name: Option<String>,
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

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PublicProfileResponse {
    pub avatar_url: String,
    pub display_name: String,
    pub username: String,
    pub bio: String,
}

impl From<account::Account> for PublicProfileResponse {
    fn from(account: account::Account) -> Self {
        PublicProfileResponse {
            username: account.username,
            avatar_url: account.avatar_url,
            bio: account.bio,
            display_name: account.display_name,
        }
    }
}
