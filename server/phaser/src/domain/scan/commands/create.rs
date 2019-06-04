use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    KernelError,
    events::EventMetadata,
};
use crate::{
    domain::scan,
    validators,
};


#[derive(Clone, Debug)]
pub struct Create {
    pub name: String,
    pub description: String,
    pub profile: scan::ScanProfile,
    pub schedule: scan::ScanSchedule,
    pub target: String,
    pub owner_id: uuid::Uuid,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Create {
    type Aggregate = scan::Scan;
    type Event = scan::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        validators::scan_name(&self.name)?;
        validators::target(&self.target)?;

        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let id = uuid::Uuid::new_v4();
        let data = scan::EventData::CreatedV1(scan::CreatedV1{
            id,
            name: self.name.clone(),
            description: self.description.clone(),
            owner_id: self.owner_id,
            profile: self.profile.clone(),
            schedule: self.schedule.clone(),
            targets: vec![self.target.clone()],
        });

        return  Ok((scan::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
