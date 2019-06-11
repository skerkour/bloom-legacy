use actix::{Message, Handler};
use serde::{Serialize, Deserialize};
use kernel::{
    db::DbActor,
    myaccount::domain,
    error::KernelError,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindAccounts {
    pub actor: domain::Account,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
}

impl Message for FindAccounts {
    type Result = Result<(Vec<domain::Account>, i64), KernelError>;
}

impl Handler<FindAccounts> for DbActor {
    type Result = Result<(Vec<domain::Account>, i64), KernelError>;

    fn handle(&mut self, msg: FindAccounts, _: &mut Self::Context) -> Self::Result {
        use kernel::db::schema::{
            kernel_accounts,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        if !msg.actor.is_admin {
            return Err(KernelError::Forbidden("Admin role is required".to_string()));
        }

        let limit = match msg.limit {
            Some(limit) => {
                if limit > 25 {
                    25
                } else {
                    limit
                }
            },
            None => 25,
        };
        let offset = msg.offset.unwrap_or(0);

        let total: i64 = kernel_accounts::dsl::kernel_accounts
            .filter(kernel_accounts::dsl::deleted_at.is_null())
            .count()
            .get_result(&conn)?;

        let account: Vec<domain::Account> = kernel_accounts::dsl::kernel_accounts
            .filter(kernel_accounts::dsl::deleted_at.is_null())
            .order_by(kernel_accounts::dsl::created_at.desc())
            .limit(limit)
            .offset(offset)
            .load(&conn)?;

        return Ok((account, total));
    }
}
