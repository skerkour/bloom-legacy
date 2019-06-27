use diesel_as_jsonb::AsJsonb;
use serde::{Deserialize, Serialize};

#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub struct EventMetadata {
    pub actor_id: Option<uuid::Uuid>,
    pub request_id: Option<uuid::Uuid>,
    pub session_id: Option<uuid::Uuid>,
}
