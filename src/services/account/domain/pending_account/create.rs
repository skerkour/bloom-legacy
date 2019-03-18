use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain::PendingAccount,
    services::account,
    services::account::notifications::emails::send_account_verification_code,
    config::Config,
    services::common::utils,
    api::middlewares::logger::RequestLogger,
};
use crate::error::KernelError;


#[derive(Clone, Debug)]
pub struct Create {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub config: Config,
    // pub logger: RequestLogger,
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

        // validate
        // build_event
        // apply event to aggregate

        let code = utils::random_digit_str(6);
        let hashed_password = bcrypt::hash(&cmd.password, account::PASSWORD_BCRYPT_COST).map_err(|_| KernelError::Bcrypt)?;
        let hashed_code = bcrypt::hash(&code, super::TOKEN_BCRYPT_COST).map_err(|_| KernelError::Bcrypt)?;


        let pending_account = PendingAccount{
            id: uuid::Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            deleted_at: None,
            version: 1,

            password: hashed_password,
            email: cmd.email.clone(),
            first_name: cmd.first_name.clone(),
            last_name: cmd.last_name.clone(),
            token: hashed_code.clone(),
            trials: 0,
        };

        send_account_verification_code(
            &cmd.config, cmd.email.as_str(), format!("{} {}", &cmd.first_name, &cmd.last_name).as_str(),
            pending_account.id.to_string().as_str(), &code,
        ).expect("error sendin email");

        diesel::insert_into(account_pending_accounts).values(&pending_account).execute(&conn)?;

        return Ok(pending_account);
    }
}

impl Create {
    pub fn validate(&self, conn: &diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>) {

    }
}
