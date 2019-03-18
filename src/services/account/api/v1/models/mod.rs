use serde::{Serialize, Deserialize};
use crate::services::account::domain::PendingAccount;

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
