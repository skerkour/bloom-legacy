mod create;

use serde::{Serialize, Deserialize};
use diesel::{Queryable};
use diesel_as_jsonb::AsJsonb;
use actix::{Message, Handler};
use failure::Error;
use crate::{
    api,
    db::DbActor,
    db::schema::account_pending_accounts,
};
use crate::error::KernelError;


pub use create::Create;


#[derive(Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "account_pending_accounts"]
pub struct PendingAccount {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

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

// pub struct GetAllPendingAccounts;

// impl Message for GetAllPendingAccounts {
//     type Result = Result<Vec<PendingAccount>, KernelError>;
// }

// impl Handler<GetAllPendingAccounts> for DbActor {
//     type Result = Result<Vec<PendingAccount>, KernelError>;

//     fn handle(&mut self, msg: GetAllPendingAccounts, _: &mut Self::Context) -> Self::Result {
//         use crate::db::schema::{
//             account_pending_accounts,
//             account_pending_accounts::dsl::*,
//         };
//         use diesel::RunQueryDsl;

//         let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;
//         let items = account_pending_accounts.load::<PendingAccount>(&conn)?;

//         Ok(items)
//     }
// }
