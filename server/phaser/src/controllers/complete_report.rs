use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    db::DbActor,
    KernelError,
};
use crate::domain;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CompleteReport {
    pub report_path: String,
    pub request_id: uuid::Uuid,
}

impl Message for CompleteReport {
    type Result = Result<(), KernelError>;
}

impl Handler<CompleteReport> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: CompleteReport, _: &mut Self::Context) -> Self::Result {

        return Ok(());
    }
}
