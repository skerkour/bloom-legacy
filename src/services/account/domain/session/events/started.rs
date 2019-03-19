use serde::{Serialize, Deserialize};

// TODO
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartedV1 {
    pub id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub token: String,
    pub ip: String,
    // Location  *domain.SessionLocation `json:"location"`
    // Device    domain.SessionDevice    `json:"device"`
}
