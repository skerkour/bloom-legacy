use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Start {
    pub account_id: uuid::Uuid,
    pub password: String,
    pub ip: String,
    pub user_agent: String,
}
