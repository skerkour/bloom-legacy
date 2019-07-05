use crate::{
  events::EventMetadata,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Event {
    pub id: uuid::Uuid,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub data: EventData,
    pub aggregate_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum EventData {
    CreatedV1(CreatedV1),
    FirstNameUpdatedV1(FirstNameUpdatedV1),
    LastNameUpdatedV1(LastNameUpdatedV1),
    PasswordUpdatedV1(PasswordUpdatedV1),
    EmailUpdatedV1(EmailUpdatedV1),
    SignInFailedV1,
    AvatarUpdatedV1(AvatarUpdatedV1),
    PasswordResetRequestedV1(PasswordResetRequestedV1),
    PasswordResetedV1(PasswordResetedV1),
    DisabledV1,
    EnabledV1,
    DeletedV1(DeletedV1),
    BioUpdatedV1(BioUpdatedV1),
    DisplayNameUpdatedV1(DisplayNameUpdatedV1),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreatedV1 {
    pub id: uuid::Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub avatar_url: String,
    pub username: String,
    pub is_admin: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FirstNameUpdatedV1 {
    pub first_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LastNameUpdatedV1 {
    pub last_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PasswordUpdatedV1 {
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EmailUpdatedV1 {
    pub email: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AvatarUpdatedV1 {
    pub avatar_url: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PasswordResetRequestedV1 {
    pub password_reset_id: uuid::Uuid,
    pub password_reset_token: String, // hashed token
    pub token_plaintext: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PasswordResetedV1 {
    pub password: String, // hashed password
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeletedV1 {
    pub password: String,
    pub random_string: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BioUpdatedV1 {
    pub bio: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DisplayNameUpdatedV1 {
    pub display_name: String,
}

impl eventsourcing::Event for Event {
    type Aggregate = super::Account;

    fn apply(&self, aggregate: &mut Self::Aggregate) {
        match self.data {
            // CreatedV1
            EventData::CreatedV1(ref data) => {
                aggregate.id = data.id;
                aggregate.created_at = self.timestamp;
                aggregate.updated_at = self.timestamp;
                aggregate.deleted_at = None;
                aggregate.version = 0;
                aggregate.avatar_url = data.avatar_url.clone();
                aggregate.email = data.email.clone();
                aggregate.first_name = data.first_name.clone();
                aggregate.is_admin = data.is_admin;
                aggregate.last_name = data.last_name.clone();
                aggregate.password = data.password.clone();
                aggregate.password_reset_id = None;
                aggregate.password_reset_token = None;
                aggregate.username = data.username.clone();
                aggregate.disabled_at = None;
                aggregate.bio = String::new();
                aggregate.display_name = data.username.clone();
            },
            // FirstNameUpdatedV1
            EventData::FirstNameUpdatedV1(ref data) => {
                aggregate.first_name = data.first_name.clone();
            },
            // LastNameUpdatedV1
            EventData::LastNameUpdatedV1(ref data) => {
                aggregate.last_name = data.last_name.clone();
            },
            // PasswordUpdatedV1
            EventData::PasswordUpdatedV1(ref data) => {
                aggregate.password = data.password.clone();
            },
            // EmailUpdatedV1
            EventData::EmailUpdatedV1(ref data) => {
                aggregate.email = data.email.clone();
            },
            // SignInFailedV1
            EventData::SignInFailedV1 => {},
            // AvatarUpdatedV1
            EventData::AvatarUpdatedV1(ref data) => {
                aggregate.avatar_url = data.avatar_url.clone();
            },
            // PasswordResetRequestedV1
            EventData::PasswordResetRequestedV1(ref data) => {
                aggregate.password_reset_id = Some(data.password_reset_id);
                aggregate.password_reset_token = Some(data.password_reset_token.clone());
            },
            // PasswordResetedV1
            EventData::PasswordResetedV1(ref data) => {
                aggregate.password = data.password.clone();
                aggregate.password_reset_id = None;
                aggregate.password_reset_token = None;
            },
            // DisabledV1
            EventData::DisabledV1 => {
                aggregate.disabled_at = Some(self.timestamp);
            },
            // EnabledV1
            EventData::EnabledV1 => {
                aggregate.disabled_at = None;
            },
            // DeletedV1
            EventData::DeletedV1(ref data) => {
                aggregate.deleted_at = Some(self.timestamp);
                aggregate.disabled_at = Some(self.timestamp);
                aggregate.first_name = data.random_string.clone();
                aggregate.last_name = data.random_string.clone();
                aggregate.email = data.random_string.clone();
                aggregate.password = data.password.clone();
                aggregate.bio = data.random_string.clone();
                aggregate.display_name = data.random_string.clone();
            },
            // BioUpdatedV1
            EventData::BioUpdatedV1(ref data) => {
                aggregate.bio = data.bio.clone();
            },
            // DisplayNameUpdatedV1
            EventData::DisplayNameUpdatedV1(ref data) => {
                aggregate.display_name = data.display_name.clone();
            },
        }
    }

    fn timestamp(&self) -> chrono::DateTime<chrono::Utc> {
        return self.timestamp;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;
    use std::path::Path;

    #[test]
    fn test_account_deleted_v1() {
        env::set_current_dir(Path::new(".."))
            .expect("changing directory for correct config file path");
        let cfg = crate::config::init();
        let db_actor_addr = crate::db::get_pool_db_conn(&cfg);

        let conn = db_actor_addr.get().unwrap();
        let mut fake_account = account::Account::new();
        fake_account.username = crate::utils::random_hex_string(10);
        let fake_request_id = uuid::Uuid::new_v4();
        let fake_session_id = uuid::Uuid::new_v4();
        let metadata = EventMetadata {
            actor_id: Some(fake_account.id),
            request_id: Some(fake_request_id.clone()),
            session_id: Some(fake_session_id.clone()),
        };
        let delete_account_cmd = account::Delete {
            metadata: metadata.clone(),
        };
        assert!(fake_account.deleted_at.is_none());

        let (account_to_delete, _event, _) =
            eventsourcing::execute(&conn, fake_account.clone(), &delete_account_cmd)
                .expect("error executing delete account command");

        assert_eq!(fake_account.id, account_to_delete.id);
        assert!(account_to_delete.deleted_at.is_some());
    }
}
