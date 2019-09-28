use crate::{
    config::Config,
    error::KernelError,
    myaccount,
    myaccount::domain::{account, pending_account},
    utils,
};
use crypto42::kdf::argon2id;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};

#[derive(Clone, Debug)]
pub struct Create {
    pub display_name: String,
    pub email: String,
    pub auth_key: String,
    pub config: Config,
}

impl eventsourcing::Command for Create {
    type Aggregate = pending_account::PendingAccount;
    type Event = Created;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        use crate::db::schema::kernel_accounts::dsl::*;
        use diesel::prelude::*;

        account::validators::display_name(&self.display_name)?;
        // account::validators::password(self.config.basic_passwords.clone(), &self.password)?;
        account::validators::email(self.config.disposable_email_domains.clone(), &self.email)?;

        // verify that an email isn't already in use
        let existing_email: i64 = kernel_accounts
            .filter(email.eq(&self.email))
            .count()
            .get_result(ctx)?;
        if existing_email != 0 {
            return Err(KernelError::Validation(format!(
                "Email: {} is already in use.",
                &self.email
            )));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        let new_pending_account_id = uuid::Uuid::new_v4();
        let verification_code = utils::random_digit_string(8);
        let auth_key_hash = argon2id::hash_password(
            self.auth_key.as_bytes(),
            myaccount::PASSWORD_ARGON2_OPSLIMIT,
            myaccount::PASSWORD_ARGON2_MEMLIMIT,
        )?
        .to_string();
        let verification_code_hash = argon2id::hash_password(
            verification_code.as_bytes(),
            myaccount::PENDING_USER_CODE_ARGON2_OPSLIMIT,
            myaccount::PENDING_USER_CODE_ARGON2_MEMLIMIT,
        )?
        .to_string();

        return Ok(Created {
            timestamp: chrono::Utc::now(),
            id: new_pending_account_id,
            display_name: self.display_name.clone(),
            email: self.email.clone(),
            auth_key_hash,
            verification_code_hash,
            verification_code,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Created {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub display_name: String,
    pub email: String,
    pub auth_key_hash: String,
    pub verification_code_hash: String,
    pub verification_code: String,
}

impl Event for Created {
    type Aggregate = pending_account::PendingAccount;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            email: self.email.clone(),
            display_name: self.display_name.clone(),
            auth_key_hash: self.auth_key_hash.clone(),
            verification_code_hash: self.verification_code_hash.clone(),
            trials: 0,
            verified: false,
        };
    }
}
