use kernel::myaccount::domain::account;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountResponse {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub first_name: String,
    pub last_name: String,
    pub username: String,
    pub email: String,
    pub avatar_url: String,
    pub is_admin: bool,
    pub disabled_at: Option<chrono::DateTime<chrono::Utc>>,
    pub bio: String,
    pub display_name: String,
}

impl From<account::Account> for AccountResponse {
    fn from(account: account::Account) -> Self {
        AccountResponse {
            id: account.id,
            created_at: account.created_at,
            first_name: account.first_name,
            last_name: account.last_name,
            username: account.username,
            email: account.email,
            avatar_url: account.avatar_url,
            is_admin: account.is_admin,
            disabled_at: account.disabled_at,
            bio: account.bio,
            display_name: account.display_name,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccountsResponse {
    pub hits: Vec<AccountResponse>,
    pub total: u64,
}
