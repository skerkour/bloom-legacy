use crate::{
    error::KernelError, events::EventMetadata, myaccount, myaccount::domain::account, utils,
};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct Delete {
    pub actor: account::Account,
}

impl eventsourcing::Command for Delete {
    type Aggregate = account::Account;
    type Event = Deleted;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if aggregate.deleted_at.is_some() {
            return Err(KernelError::Validation(
                "Account is already deleted".to_string(),
            ));
        }

        if !self.actor.is_admin {
            return Err(KernelError::Forbidden("Admin role is required".to_string()));
        }

        Ok(())
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        let random_string = utils::random_hex_string(5);
        let random_password = utils::random_hex_string(128);
        let password = bcrypt::hash(random_password, myaccount::PASSWORD_BCRYPT_COST)
            .map_err(|_| KernelError::Bcrypt)?;

        return Ok(Deleted {
            timestamp: chrono::Utc::now(),
            random_string,
            password,
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct Deleted {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub password: String,
    pub random_string: String,
}

impl Event for Deleted {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            deleted_at: Some(self.timestamp),
            disabled_at: Some(self.timestamp),
            first_name: self.random_string.clone(),
            last_name: self.random_string.clone(),
            email: self.random_string.clone(),
            password: self.password.clone(),
            bio: self.random_string.clone(),
            display_name: self.random_string.clone(),
            ..aggregate
        };
    }
}


// #[cfg(test)]
// mod test {
//     use super::*;
//     use std::env;
//     use std::path::Path;

//     #[test]
//     fn test_account_deleted_v1() {
//         env::set_current_dir(Path::new(".."))
//             .expect("changing directory for correct config file path");
//         let cfg = crate::config::init();
//         let db_actor_addr = crate::db::get_pool_db_conn(&cfg);

//         let conn = db_actor_addr.get().unwrap();
//         let mut fake_account = account::Account::new();
//         fake_account.username = crate::utils::random_hex_string(10);
//         let fake_request_id = uuid::Uuid::new_v4();
//         let fake_session_id = uuid::Uuid::new_v4();
//         let metadata = EventMetadata {
//             actor_id: Some(fake_account.id),
//             request_id: Some(fake_request_id.clone()),
//             session_id: Some(fake_session_id.clone()),
//         };
//         let delete_account_cmd = account::Delete {
//             metadata: metaself.clone(),
//         };
//         assert!(fake_account.deleted_at.is_none());

//         let (account_to_delete, _event, _) =
//             eventsourcing::execute(&conn, fake_account.clone(), &delete_account_cmd)
//                 .expect("error executing delete account command");

//         assert_eq!(fake_account.id, account_to_delete.id);
//         assert!(account_to_delete.deleted_at.is_some());
//     }
// }
