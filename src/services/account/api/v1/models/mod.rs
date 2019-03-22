use serde::{Serialize, Deserialize};


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
pub struct VerifyResponse {
    pub is_valid: bool,
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
pub struct SignOutResponse {}

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
