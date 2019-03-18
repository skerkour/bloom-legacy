use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain::PendingAccount,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Verify {
    pub id: String,
    pub code: String,
}

impl Message for Verify {
    type Result = Result<(), KernelError>;
}

// TODO: transaction
impl Handler<Verify> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, cmd: Verify, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::account_pending_accounts::dsl::*;
        use diesel::RunQueryDsl;
        use diesel::query_dsl::filter_dsl::FilterDsl;
        use diesel::ExpressionMethods;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        // verify token is valdi
        // verify token hasn't expired

        let aggregate_id = uuid::Uuid::parse_str(&cmd.id).map_err(|_| KernelError::Validation("id is not a valid uuid".to_string()))?;

        let pending_account: Result<PendingAccount, _> = account_pending_accounts
            .filter(id.eq(aggregate_id))
            .first(&conn);

        match pending_account {
            Ok(_) => return Ok(()),
            Err(_) => return Err(KernelError::Validation("pending account not found".to_string())),
        }

        // return Ok(());
    }
}


impl Verify {
    pub fn validate(&self, _conn: &diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>) -> Result<(), KernelError> {
        // verify given code

        // verify code expiration

        return Ok(());
    }
}
