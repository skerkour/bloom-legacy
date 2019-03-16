use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain::PendingAccount,
    services::account::notifications::emails::send_account_verification_code,
    config::Config,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Create {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub config: Config,
}

impl Message for Create {
    type Result = Result<PendingAccount, KernelError>;
}

impl Handler<Create> for DbActor {
    type Result = Result<PendingAccount, KernelError>;

    fn handle(&mut self, cmd: Create, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::account_pending_accounts::dsl::*;
        use diesel::RunQueryDsl;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;
        let pending_account = PendingAccount{
            id: uuid::Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
            version: 1,

            password: cmd.password, // hashed password
            email: cmd.email.clone(),
            first_name: cmd.first_name.clone(),
            last_name: cmd.last_name.clone(),
            token: "".to_string(), // hashed code
        };

        send_account_verification_code(
            &cmd.config, cmd.email.as_str(), cmd.first_name.as_str(),
            cmd.last_name.as_str(), pending_account.id.to_string().as_str(), "122221",
        ).expect("error sendin email");

        diesel::insert_into(account_pending_accounts).values(&pending_account).execute(&conn)?;

        Ok(pending_account)
    }
}
