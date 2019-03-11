use serde::{Serialize, Deserialize};

// type Account struct {
// 	goes.BaseAggregate
// 	Username      string
// 	Password      string // hashed password
// 	Email         string
// 	FirstName     string
// 	LastName      string
// 	Avatar        string
// 	RecoveryToken *string
// 	RecoveryID    *string
// 	IsAdmin       bool
// }

#[derive(Deserialize, Serialize)]
pub struct Account {
    pub username: String,
    pub password: String, // hashed password
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub avatar_url: String,
    pub recovery_token: Option<String>,
    pub recovery_id: Option<String>,
    pub is_admin: bool,
}
