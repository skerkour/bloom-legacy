use crate::domain::scan;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use eventsourcing::{Event, EventTs};
use kernel::KernelError;

#[derive(Clone, Debug)]
pub struct Create {
    pub name: String,
    pub description: String,
    pub profile: scan::ScanProfile,
    pub schedule: scan::ScanSchedule,
    pub target: String,
    pub owner_id: uuid::Uuid,
}

impl eventsourcing::Command for Create {
    type Aggregate = scan::Scan;
    type Event = Created;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        scan::validators::scan_name(&self.name)?;
        scan::validators::target(&self.target)?;

        return Ok(());
    }

    fn build_event(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        return Ok(Created {
            timestamp: chrono::Utc::now(),
            id: uuid::Uuid::new_v4(),
            name: self.name.clone(),
            description: self.description.clone(),
            owner_id: self.owner_id,
            profile: self.profile.clone(),
            schedule: self.schedule.clone(),
            targets: vec![self.target.clone()],
        });
    }
}

// Event
#[derive(Clone, Debug, EventTs)]
pub struct Created {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub id: uuid::Uuid,
    pub owner_id: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub profile: scan::ScanProfile,
    pub schedule: scan::ScanSchedule,
    pub targets: Vec<String>,
}

impl Event for Created {
    type Aggregate = scan::Scan;

    fn apply(&self, _aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            id: self.id,
            created_at: self.timestamp,
            updated_at: self.timestamp,
            version: 0,
            name: self.name.clone(),
            description: self.description.clone(),
            findings: 0,
            last: None,
            profile: self.profile.clone(),
            schedule: self.schedule.clone(),
            state: scan::ScanState::Waiting,
            targets: self.targets.clone(),
            owner_id: self.owner_id,
        };
    }
}
