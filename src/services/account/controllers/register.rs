use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::account::domain,
    services::account,
    services::account::domain::pending_account,
    services::account::notifications::emails::send_account_verification_code,
    config::Config,
    services::common::utils,
    error::KernelError,
};


#[derive(Clone, Debug)]
pub struct Register {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub config: Config,
    // pub logger: RequestLogger,
}

impl Message for Register {
    type Result = Result<domain::PendingAccount, KernelError>;
}

impl Handler<Register> for DbActor {
    type Result = Result<domain::PendingAccount, KernelError>;

    fn handle(&mut self, msg: Register, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            account_pending_accounts::dsl::*,
            account_pending_accounts_events::dsl::*,
        };
        use diesel::RunQueryDsl;

        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;
        let config = msg.config.clone();
        let cmd = pending_account::Create::from(msg);

        // validate
        cmd.validate(&conn)?;


        let code = utils::random_digit_string(8);
        let pending_account_id = uuid::Uuid::new_v4();
        let hashed_password = bcrypt::hash(&cmd.password, account::PASSWORD_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;
        let hashed_code = bcrypt::hash(&code, pending_account::TOKEN_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;
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


        let event = pending_account::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: now,
            data: pending_account::EventData::CreatedV1(created),
            aggregate_id: pending_account_id,
            metadata: pending_account::EventMetadata{},
        };

        diesel::insert_into(account_pending_accounts)
            .values(&pending_account)
            .execute(&conn)?;
        diesel::insert_into(account_pending_accounts_events)
            .values(&event)
            .execute(&conn)?;

        send_account_verification_code(
            &config,
            cmd.email.as_str(),
            format!("{} {}", &cmd.first_name, &cmd.last_name).as_str(),
            pending_account.id.to_string().as_str(),
            &code,
        ).expect("error sending email");

        return Ok(pending_account);
    }
}
