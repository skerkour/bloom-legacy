use crate::{error::KernelError, accounts::domain::pending_email};
use chrono::Utc;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};
use crypto42::kdf::argon2id;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Verify {
    pub id: uuid::Uuid,
    pub code: String,
    pub email: String,
}

impl eventsourcing::Command for Verify {
    type Aggregate = pending_email::PendingEmail;
    type Event = Verified;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        use crate::db::schema::kernel_accounts::dsl::*;
        use diesel::prelude::*;

        // validators::email(&self.email)?; already done ine pending email create

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
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        let timestamp = Utc::now();
        let duration = aggregate.created_at.signed_duration_since(timestamp);

        if aggregate.trials + 1 >= 8 {
            return Err(KernelError::Validation(
                pending_email::VerificationFailedReason::TooManyTrials.to_string(),
            ));
        } else if !argon2id::verify_password(aggregate.token.as_str().into(), &self.code.as_bytes()) {
            // verify given code
            return Err(KernelError::Validation(
                pending_email::VerificationFailedReason::CodeNotValid.to_string(),
            ));
        } else if duration.num_minutes() > 30 {
            // verify code expiration
            return Err(KernelError::Validation(
                pending_email::VerificationFailedReason::CodeExpired.to_string(),
            ));
        }

        return Ok(Verified {
            timestamp: chrono::Utc::now(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct Verified {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Verified {
    type Aggregate = pending_email::PendingEmail;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return aggregate;
    }
}
