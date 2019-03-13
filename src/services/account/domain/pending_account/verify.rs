use actix::{Message, Handler};
use crate::{
    db::DbActor,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Verify {
    pub token: String,
}

impl Message for Verify {
    type Result = Result<(), KernelError>;
}

impl Handler<Verify> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: Verify, _: &mut Self::Context) -> Self::Result {
        // use crate::db::schema::account_pending_accounts::dsl::*;
        // use diesel::RunQueryDsl;

        // verify token is valdi
        // verify token hasn't expired

        Ok(())
    }
}
