use actix::{Handler, Message};
use kernel::{db::DbActor, error::KernelError, myaccount::domain};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FindAccounts {
    pub actor: domain::Account,
    pub offset: Option<i64>,
    pub limit: Option<i64>,
    pub username: Option<String>,
    pub email: Option<String>,
}

impl Message for FindAccounts {
    type Result = Result<(Vec<domain::Account>, i64), KernelError>;
}

impl Handler<FindAccounts> for DbActor {
    type Result = Result<(Vec<domain::Account>, i64), KernelError>;

    fn handle(&mut self, msg: FindAccounts, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::kernel_accounts;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

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
            }
            None => 25,
        };
        let offset = msg.offset.unwrap_or(0);

        return match (msg.email, msg.username) {
            (Some(ref email), Some(ref username)) => {
                let total: i64 = kernel_accounts::dsl::kernel_accounts
                    .filter(kernel_accounts::dsl::username.eq(username))
                    .filter(kernel_accounts::dsl::email.eq(email))
                    .count()
                    .get_result(&conn)?;
                let accounts: Vec<domain::Account> = kernel_accounts::dsl::kernel_accounts
                    .filter(kernel_accounts::dsl::username.eq(username))
                    .filter(kernel_accounts::dsl::email.eq(email))
                    .limit(limit)
                    .offset(offset)
                    .load(&conn)?;
                Ok((accounts, total))
            }
            (Some(ref email), None) => {
                let total: i64 = kernel_accounts::dsl::kernel_accounts
                    .filter(kernel_accounts::dsl::email.eq(email))
                    .count()
                    .get_result(&conn)?;
                let accounts: Vec<domain::Account> = kernel_accounts::dsl::kernel_accounts
                    .filter(kernel_accounts::dsl::email.eq(email))
                    .limit(limit)
                    .offset(offset)
                    .load(&conn)?;
                Ok((accounts, total))
            }
            (None, Some(ref username)) => {
                let total: i64 = kernel_accounts::dsl::kernel_accounts
                    .filter(kernel_accounts::dsl::username.eq(username))
                    .count()
                    .get_result(&conn)?;
                let accounts: Vec<domain::Account> = kernel_accounts::dsl::kernel_accounts
                    .filter(kernel_accounts::dsl::username.eq(username))
                    .limit(limit)
                    .offset(offset)
                    .load(&conn)?;
                Ok((accounts, total))
            }
            (None, None) => {
                let total: i64 = kernel_accounts::dsl::kernel_accounts
                    .count()
                    .get_result(&conn)?;

                let accounts: Vec<domain::Account> = kernel_accounts::dsl::kernel_accounts
                    .order_by(kernel_accounts::dsl::created_at.desc())
                    .limit(limit)
                    .offset(offset)
                    .load(&conn)?;
                Ok((accounts, total))
            }
        };
    }
}
