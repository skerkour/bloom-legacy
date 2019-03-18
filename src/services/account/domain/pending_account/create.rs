use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain,
    services::account,
    services::account::validators,
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
    type Result = Result<domain::PendingAccount, KernelError>;
}

impl Handler<Create> for DbActor {
    type Result = Result<domain::PendingAccount, KernelError>;

    fn handle(&mut self, cmd: Create, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            account_pending_accounts::dsl::*,
            account_pending_accounts_events::dsl::*,
        };
        use diesel::RunQueryDsl;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        // validate
        cmd.validate(&conn)?;


        let code = utils::random_digit_str(8);
        let pending_account_id = uuid::Uuid::new_v4();
        let hashed_password = bcrypt::hash(&cmd.password, account::PASSWORD_BCRYPT_COST).map_err(|_| KernelError::Bcrypt)?;
        let hashed_code = bcrypt::hash(&code, super::TOKEN_BCRYPT_COST).map_err(|_| KernelError::Bcrypt)?;
        let now = chrono::Utc::now();


        // build_event
        let created = domain::pending_account::CreatedV1{
            id: pending_account_id.clone(),
            password: hashed_password.clone(),
            email: cmd.email.clone(),
            first_name: cmd.first_name.clone(),
            last_name: cmd.last_name.clone(),
            token: hashed_code.clone(),
            trials: 0,
        };

        // apply event to aggregate
        let pending_account = domain::PendingAccount{
            id: pending_account_id.clone(),
            created_at: now.clone(),
            updated_at: now.clone(),
            deleted_at: None,
            version: 1,

            password: hashed_password.clone(),
            email: cmd.email.clone(),
            first_name: cmd.first_name.clone(),
            last_name: cmd.last_name.clone(),
            token: hashed_code.clone(),
            trials: 0,
        };


        let event = super::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: now,
            data: super::EventData::CreatedV1(created),
            aggregate_id: pending_account_id,
            metadata: super::EventMetadata{},
        };

        diesel::insert_into(account_pending_accounts).values(&pending_account).execute(&conn)?;
        diesel::insert_into(account_pending_accounts_events).values(&event).execute(&conn)?;

        send_account_verification_code(
            &cmd.config, cmd.email.as_str(), format!("{} {}", &cmd.first_name, &cmd.last_name).as_str(),
            pending_account.id.to_string().as_str(), &code,
        ).expect("error sendin email");

        return Ok(pending_account);
    }
}

impl Create {
    pub fn validate(&self, _conn: &diesel::r2d2::PooledConnection<diesel::r2d2::ConnectionManager<diesel::PgConnection>>) -> Result<(), KernelError> {
        validators::first_name(&self.first_name)?;
        validators::last_name(&self.last_name)?;
        validators::password(&self.password)?;
        // TODO: validate email

        if self.password == self.email {
            return Err(KernelError::Validation("password must be different than your email address".to_string()));
        }

        return Ok(());
    }
}
