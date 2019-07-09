use crate::{
    config::Config,
    error::KernelError,
    myaccount,
    myaccount::domain::{account, pending_account},
    utils,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};

#[derive(Clone, Debug)]
pub struct Create {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
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

        account::validators::first_name(&self.first_name)?;
        account::validators::last_name(&self.last_name)?;
        account::validators::password(self.config.basic_passwords.clone(), &self.password)?;
        account::validators::email(self.config.disposable_email_domains.clone(), &self.email)?;

        if self.password == self.email {
            return Err(KernelError::Validation(
                "password must be different than your email address".to_string(),
            ));
        }

        // verify that an email isn't already in use
        let existing_email: i64 = kernel_accounts
            .filter(email.eq(&self.email))
            .filter(deleted_at.is_null())
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
        let code = utils::random_digit_string(8);
        let hashed_password = bcrypt::hash(&self.password, myaccount::PASSWORD_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;
        let token = bcrypt::hash(&code, myaccount::PENDING_USER_TOKEN_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        return Ok(Created {
            timestamp: chrono::Utc::now(),
            id: new_pending_account_id,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            password: hashed_password,
            token,
            code,
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Created {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub token: String,
    pub code: String,
}

impl Event for Created {
    type Aggregate = pending_account::PendingAccount;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            deleted_at: None,
            version: 0,
            email: self.email.clone(),
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            password: self.password.clone(),
            token: self.token.clone(),
            trials: 0,
            verified: false,
        };
    }
}
