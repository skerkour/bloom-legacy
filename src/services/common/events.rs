use serde::{Serialize, Deserialize};
use diesel_as_jsonb::AsJsonb;


#[derive(AsJsonb, Clone, Debug, Deserialize, Serialize)]
pub struct EventMetadata {
    pub actor_id: Option<uuid::Uuid>,
    pub request_id: Option<String>,
}
