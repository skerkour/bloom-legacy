use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain::PendingAccount,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CodeResentV1 {
    pub token: String,
}
