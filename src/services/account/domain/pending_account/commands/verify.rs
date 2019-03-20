use actix::{Message, Handler};
use crate::{
    db::DbActor,
    services::{
        account::{
            domain::{PendingAccount, pending_account::EventData},
            controllers,
        },
    },
};
use crate::error::KernelError;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Verify {
    pub id: String,
    pub code: String,
}


impl Verify {
    pub fn validate(&self, _conn: &PooledConnection<ConnectionManager<PgConnection>>, aggregate: &PendingAccount) -> Result<(), KernelError> {
        let now = Utc::now();

        if aggregate.trials + 1 >= 10 {
            return Err(KernelError::Validation("Maximum number of trials reached. Please create another account.".to_string()));
        }

        // verify given code
        let is_valid = bcrypt::verify(&self.code, &aggregate.token).map_err(|_| KernelError::Bcrypt)?;
        if !is_valid {
            return Err(KernelError::Validation("Code is not valid.".to_string()));
        }

        // verify code expiration
        let duration = aggregate.created_at.signed_duration_since(now);
        if duration.num_minutes() > 30 {
            return Err(KernelError::Validation("Code has expired. Please create another account.".to_string()));
        }

        return Ok(());
    }
}
