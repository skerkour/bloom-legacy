use actix::{Message, Handler};
use crate::{
    db::DbActor,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteRegistration {
    pub code: String,
}

impl Message for CompleteRegistration {
    type Result = Result<bool, KernelError>;
}

impl Handler<CompleteRegistration> for DbActor {
    type Result = Result<bool, KernelError>;

    fn handle(&mut self, msg: CompleteRegistration, _: &mut Self::Context) -> Self::Result {
        // use crate::db::schema::account_pending_accounts::dsl::*;
        // use diesel::RunQueryDsl;

        // verify token is valdi
        // verify token hasn't expired

        return Ok(false);
    }
}
