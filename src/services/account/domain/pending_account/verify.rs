use actix::{Message, Handler};
use crate::{
    db::DbActor,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};

// verify that the given code is valid or not
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Verify {
    pub code: String,
}

impl Message for Verify {
    type Result = Result<bool, KernelError>;
}

impl Handler<Verify> for DbActor {
    type Result = Result<bool, KernelError>;

    fn handle(&mut self, msg: Verify, _: &mut Self::Context) -> Self::Result {
        // use crate::db::schema::account_pending_accounts::dsl::*;
        // use diesel::RunQueryDsl;

        // verify token is valdi
        // verify token hasn't expired

        return Ok(false);
    }
}
