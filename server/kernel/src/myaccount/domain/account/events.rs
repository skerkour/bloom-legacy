use crate::{
  events::EventMetadata,
};
use serde::{Deserialize, Serialize};
use eventsourcing::{Event, EventTs};

// CreatedV1
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct CreatedV1 {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: String,
    pub username: String,
    pub is_admin: bool,
}

impl Event for CreatedV1 {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            deleted_at: None,
            version: 0,
            avatar_url: self.avatar_url.clone(),
            email: self.email.clone(),
            first_name: self.first_name.clone(),
            is_admin: self.is_admin,
            last_name: self.last_name.clone(),
            password: self.password.clone(),
            password_reset_id: None,
            password_reset_token: None,
            username: self.username.clone(),
            disabled_at: None,
            bio: String::new(),
            display_name: self.username.clone(),
        };
    }
}


// FirstNameUpdatedV1
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct FirstNameUpdatedV1 {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub first_name: String,
}

impl Event for FirstNameUpdatedV1 {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            first_name: self.first_name.clone(),
            ..aggregate
        };
    }
}


// LastNameUpdatedV1
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct LastNameUpdatedV1 {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub last_name: String,
}

impl Event for LastNameUpdatedV1 {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            last_name: self.last_name.clone(),
            ..aggregate
        };
    }
}

// PasswordUpdatedV1
#[derive(Clone, Debug, Deserialize, Eventts, Serialize)]
pub struct PasswordUpdatedV1 {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub password: String,
}

impl Event for PasswordUpdatedV1 {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            password: self.password.clone(),
            ..aggregate
        };
    }
}

// EmailUpdatedV1
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct EmailUpdatedV1 {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub email: String,
}

impl Event for EmailUpdatedV1 {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            email: self.email.clone(),
            ..aggregate
        };
    }
}

// AvatarUpdatedV1
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct AvatarUpdatedV1 {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub avatar_url: String,
}

impl Event for AvatarUpdatedV1 {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            avatar_url: self.avatar_url.clone(),
            ..aggregate
        };
    }
}

// PasswordResetRequestedV1
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct PasswordResetRequestedV1 {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub password_reset_id: uuid::Uuid,
    pub password_reset_token: String, // hashed token
    pub token_plaintext: String,
}

impl Event for PasswordResetRequestedV1 {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
           password_reset_id: Some(self.password_reset_id),
            password_reset_token: Some(self.password_reset_token.clone()),
            ..aggregate
        };
    }
}

// PasswordResetedV1
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct PasswordResetedV1 {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub password: String, // hashed password
}

impl Event for PasswordResetedV1 {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
           password: self.password.clone(),
                password_reset_id: None,
                password_reset_token: None,
            ..aggregate
        };
    }
}

// DisabledV1
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct DisabledV1 {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for DisabledV1 {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
           disabled_at: Some(self.timestamp),
            ..aggregate
        };
    }
}

// EnabledV1
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct EnabledV1 {
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl Event for EnabledV1 {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
           disabled_at: None,
            ..aggregate
        };
    }
}

// DeletedV1
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct DeletedV1 {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub password: String,
    pub random_string: String,
}

impl Event for DeletedV1 {
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

// BioUpdatedV1
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct BioUpdatedV1 {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub bio: String,
}

impl Event for BioUpdatedV1 {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
           bio: self.bio.clone(),
            ..aggregate
        };
    }
}

// DisplayNameUpdatedV1
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct DisplayNameUpdatedV1 {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub display_name: String,
}

impl Event for DisplayNameUpdatedV1 {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
           display_name: self.display_name.clone(),
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
