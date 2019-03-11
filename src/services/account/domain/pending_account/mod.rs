use serde::{Serialize, Deserialize};

// type PendingAccount struct {
// 	goes.BaseAggregate
// 	FirstName string `json:"first_name"`
// 	LastName  string `json:"last_name"`
// 	Email     string `json:"email"`
// 	Password  string `json:"password"`
// 	Token     string `json:"token"` // hashed token
// }

// // TableName to indicate to gorm which postgres table to use
// func (PendingAccount) TableName() string {
// 	return "account_pending_accounts"
// }

// // AggregateType to implement the goes.Aggregate interface
// func (PendingAccount) AggregateType() string {
// 	return "account_pending_account"
// }


#[derive(Deserialize, Serialize)]
pub struct PendingAccount {
    pub password: String, // hashed password
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub token: String, // hashed token
}
