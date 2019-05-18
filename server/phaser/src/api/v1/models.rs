use serde::{Serialize, Deserialize};
use crate::domain::{
    scan,
    report,
};


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ScanResponse {
    pub id: uuid::Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub description: String,
    pub findings: i64,
    pub last: Option<chrono::DateTime<chrono::Utc>>,
    pub name: String,
    pub profile: scan::ScanProfile,
    pub schedule: scan::ScanSchedule,
    pub state: scan::ScanState,
    pub target: String,
}

impl From<scan::Scan> for ScanResponse {
    fn from(scan: scan::Scan) -> Self {
        Self{
            id: scan.id,
            created_at: scan.created_at,
            description: scan.description,
            findings: scan.findings,
            last: scan.last,
            name: scan.name,
            profile: scan.profile,
            schedule: scan.schedule,
            state: scan.state,
            target: scan.targets[0].clone(),
        }
    }
}


#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CreateScanBody {
    pub description: String,
    pub name: String,
    pub profile: scan::ScanProfile,
    pub schedule: scan::ScanSchedule,
    pub target: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReportJob {
    pub id: uuid::Uuid,
    pub scan_id: uuid::Uuid,
    pub targets: Vec<String>,
    pub profile: scan::ScanProfile,
}

impl From<report::Report> for ReportJob {
    fn from(report: report::Report) -> Self {
        Self{
            id: report.id,
            scan_id: report.scan_id,
            targets: report.targets,
            profile:report.profile,
        }
    }
}
