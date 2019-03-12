use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;

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

// 	ID            string      `json:"id"`
// 	Timestamp     time.Time   `json:"timestamp"`
// 	AggregateID   string      `json:"aggregate_id"`
// 	AggregateType string      `json:"aggregate_type"`
// 	Action        string      `json:"action"`
// 	Version       uint64      `json:"version"`
// 	Type          string      `json:"type"`
// 	Data          interface{} `json:"data"`
// 	Metadata      Metadata    `json:"metadata"`
// NonPersisted interface{} `json:"-"`

#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct PendingAccount {
    pub password: String, // hashed password
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub token: String, // hashed token
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PendingAccountEvent {
    pub id: String,
    pub data: PendingAccountEventData,
    pub aggregate_id: String,
    pub metadata: String, // TODO: change
}




#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PendingAccountCommand {
    Create,
    ResendCode,
    Verify,
}

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub enum PendingAccountEventData {
    CreatedV1,
    CodeResentV1,
    VerifiedV1,
}
