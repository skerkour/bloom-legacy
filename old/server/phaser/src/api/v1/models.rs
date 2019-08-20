use crate::domain::{report, scan};
use serde::{Deserialize, Serialize};

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
        Self {
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
        Self {
            id: report.id,
            scan_id: report.scan_id,
            targets: report.targets,
            profile: report.profile,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ReportResponse {
    pub id: uuid::Uuid,
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    pub error: Option<String>,
    pub findings: Option<Vec<report::Finding>>,
    pub high_level_findings: i64,
    pub information_findings: i64,
    pub low_level_findings: i64,
    pub medium_level_findings: i64,
    pub profile: scan::ScanProfile,
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    pub status: report::ReportStatus,
    pub target: String,
    pub total_findings: i64,
    pub trigger: scan::ReportTrigger,
    pub scan_id: uuid::Uuid,
}

impl From<report::Report> for ReportResponse {
    fn from(report: report::Report) -> Self {
        Self {
            id: report.id,
            completed_at: report.completed_at,
            error: report.error,
            findings: report.findings,
            high_level_findings: report.high_level_findings,
            information_findings: report.information_findings,
            low_level_findings: report.low_level_findings,
            medium_level_findings: report.medium_level_findings,
            profile: report.profile,
            started_at: report.started_at,
            status: report.status,
            target: report.targets[0].clone(),
            total_findings: report.total_findings,
            trigger: report.trigger,
            scan_id: report.scan_id,
        }
    }
}
