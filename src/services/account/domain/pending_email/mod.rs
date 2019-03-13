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


#[derive(Clone, Debug, Deserialize, Queryable, Serialize)]
pub struct PendingEmail {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub version: i64,

    pub email: String,
    pub token: String, // hashed token

    pub account_id: uuid::Uuid,
}
