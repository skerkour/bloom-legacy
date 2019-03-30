use actix::{Message, Handler};
use crate::{
    db::DbActor,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResendCode {
}

impl Message for ResendCode {
    type Result = Result<(), KernelError>;
}

impl Handler<ResendCode> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, _msg: ResendCode, _: &mut Self::Context) -> Self::Result {
        Ok(())
    }
}
