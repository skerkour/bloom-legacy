use crate::{
    config, error::KernelError, myaccount, myaccount::domain::pending_email, myaccount::validators,
    utils,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};

#[derive(Clone, Debug)]
pub struct Create {
    pub email: String,
    pub account_id: uuid::Uuid,
    pub config: config::Config,
}

impl eventsourcing::Command for Create {
    type Aggregate = pending_email::PendingEmail;
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

        validators::email(self.config.disposable_email_domains.clone(), &self.email)?;

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
        let new_pending_email_id = uuid::Uuid::new_v4();
        let code = utils::random_digit_string(8);
        let token = bcrypt::hash(&code, myaccount::PENDING_EMAIL_TOKEN_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        return Ok(Created {
            timestamp: chrono::Utc::now(),
            id: new_pending_email_id,
            email: self.email.clone(),
            account_id: self.account_id,
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
    pub email: String,
    pub account_id: uuid::Uuid,
    pub token: String,
    pub code: String,
}

impl Event for Created {
    type Aggregate = pending_email::PendingEmail;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            deleted_at: None,
            version: 0,
            email: self.email.clone(),
            token: self.token.clone(),
            trials: 0,
            account_id: self.account_id,
        };
    }
}
