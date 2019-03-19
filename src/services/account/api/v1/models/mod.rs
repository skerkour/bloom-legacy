use serde::{Serialize, Deserialize};
use crate::services::account::domain::{
    PendingAccount,
    Session,
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
    pub id: String,
}

impl From<PendingAccount> for RegisterResponse {
    fn from(pending_account: PendingAccount) -> Self {
        return RegisterResponse{
            id: pending_account.id.to_string(),
        };
    }
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
    pub id: String,
    pub token: String,
}

impl From<Session> for CompleteRegistrationResponse {
    fn from(session: Session) -> Self {
        return CompleteRegistrationResponse{
            id: session.id.to_string(),
            token: session.token,
        };
    }
}
