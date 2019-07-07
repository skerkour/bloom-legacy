use crate::{
    config::Config, error::KernelError, events::EventMetadata, myaccount,
    myaccount::domain::pending_account, myaccount::validators, utils,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};

#[derive(Clone, Debug)]
pub struct Create {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub metadata: EventMetadata,
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

        validators::first_name(&self.first_name)?;
        validators::last_name(&self.last_name)?;
        validators::password(self.config.basic_passwords.clone(), &self.password)?;
        validators::email(self.config.disposable_email_domains.clone(), &self.email)?;

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
        let now = chrono::Utc::now();
        let new_pending_account_id = uuid::Uuid::new_v4();
        let code = utils::random_digit_string(8);
        let hashed_password = bcrypt::hash(&self.password, myaccount::PASSWORD_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;
        let token = bcrypt::hash(&code, myaccount::PENDING_USER_TOKEN_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        let data = pending_account::EventData::CreatedV1(pending_account::CreatedV1 {
            id: new_pending_account_id,
            first_name: self.first_name.clone(),
            last_name: self.last_name.clone(),
            email: self.email.clone(),
            password: hashed_password,
            token,
            code,
        });

        return Ok(pending_account::Event {
            id: uuid::Uuid::new_v4(),
            timestamp: now,
            data,
            aggregate_id: new_pending_account_id,
            metadata: self.metadata.clone(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
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

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: data.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            deleted_at: None,
            version: 0,
            email: data.email.clone(),
            first_name: data.first_name.clone(),
            last_name: data.last_name.clone(),
            password: data.password.clone(),
            token: data.token.clone(),
            trials: 0,
            verified: false,
        };
    }
}
