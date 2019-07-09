use crate::{error::KernelError, myaccount::domain::account};
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
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if !self.actor.is_admin {
            return Err(KernelError::Forbidden("Admin role is required".to_string()));
        }

        Ok(())
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Deleted {
            timestamp: chrono::Utc::now(),
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct Deleted {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for Deleted {
    type Aggregate = account::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return aggregate;
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
//         let mut account_to_delete = account::Account::new();
//         account_to_delete.username = crate::utils::random_hex_string(10);
//         let delete_account_cmd = account::Delete {};
//        ASSERT

//         let (account_to_delete, _event) =
//             eventsourcing::execute(&conn, account_to_delete, &delete_account_cmd)
//                 .expect("error executing delete account command");
//        ASSERT
//     }
// }
