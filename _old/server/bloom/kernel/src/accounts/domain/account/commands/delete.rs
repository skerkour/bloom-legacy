use crate::{error::KernelError, accounts::domain::account};
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};

#[derive(Clone)]
pub struct Delete {
    pub s3_client: rusoto_s3::S3Client,
    pub s3_bucket: String,
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
        aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Deleted {
            id: aggregate.id,
            timestamp: chrono::Utc::now(),
            s3_client: self.s3_client.clone(),
            s3_bucket: self.s3_bucket.clone(),
        });
    }
}

// Event
#[derive(Clone, EventTs)]
pub struct Deleted {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub s3_client: rusoto_s3::S3Client,
    pub s3_bucket: String,
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
