use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain::PendingAccount,
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Create {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl Message for Create {
    type Result = Result<PendingAccount, KernelError>;
}

impl Handler<Create> for DbActor {
    type Result = Result<PendingAccount, KernelError>;

    fn handle(&mut self, msg: Create, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::account_pending_accounts::dsl::*;
        use diesel::RunQueryDsl;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;
        let pending_account = PendingAccount{
            id: uuid::Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
            version: 1,

            password: msg.password, // hashed password
            email: msg.email,
            first_name: msg.first_name,
            last_name: msg.last_name,
            token: "".to_string(), // hashed token
        };

        diesel::insert_into(account_pending_accounts).values(&pending_account).execute(&conn)?;

        Ok(pending_account)
    }
}
