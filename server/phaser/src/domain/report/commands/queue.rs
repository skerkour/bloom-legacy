use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    KernelError,
    events::EventMetadata,
};
use crate::{
    domain::report,
    domain::scan,
};


#[derive(Clone, Debug)]
pub struct Queue {
    pub scan_id: uuid::Uuid,
    pub targets: Vec<String>,
    pub profile: scan::ScanProfile,
    pub trigger: scan::ReportTrigger,
    pub metadata: EventMetadata,
}

impl eventsourcing::Command for Queue {
    type Aggregate = report::Report;
    type Event = report::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        return Ok(());
    }

    fn build_event(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let id = uuid::Uuid::new_v4();
        let data = report::EventData::QueuedV1(report::QueuedV1{
            id: id,
            scan_id: self.scan_id,
            targets: self.targets.clone(),
            profile: self.profile.clone(),
            trigger: self.trigger.clone(),
        });

        return  Ok((report::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
