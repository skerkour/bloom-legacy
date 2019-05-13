use serde::{Serialize, Deserialize};
use crate::domain::scan;


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScanResponse {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl From<scan::Scan> for ScanResponse {
    fn from(scan: scan::Scan) -> Self {
        Self{
            id: scan.id,
            created_at: scan.created_at,
        }
    }
}
