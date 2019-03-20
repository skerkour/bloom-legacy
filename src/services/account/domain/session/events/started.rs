use serde::{Serialize, Deserialize};
use crate::services::account::domain::session::{Location, Device};

// TODO
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct StartedV1 {
    pub id: uuid::Uuid,
    pub account_id: uuid::Uuid,
    pub token: String,
    pub ip: String,
    pub location: Location,
    pub device: Device,
}

// #[derive(Clone, Debug, Deserialize, Serialize)]
// pub struct StartedV1NonPersisted {
//     pub token: String,
// }
