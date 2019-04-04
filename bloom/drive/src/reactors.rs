use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    KernelError,
    accounts::domain::account,
};
use crate::{
    domain::{
        file,
        profile,
    },
};




pub struct AccountCreated;
impl eventsourcing::Subscription for AccountCreated {
    type Error = KernelError;
    type Message = account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;

    fn handle(&self, ctx: &Self::Context, msg: &Self::Message) -> Result<(), Self::Error> {
        use kernel::db::schema::{
            drive_files,
            drive_files_events,
        };
        use diesel::prelude::*;
        // create home

        if let account::EventData::CreatedV1(ref data) = msg.data {
            let metadata = msg.metadata.clone();
            let create_cmd = file::Create{
                name: "__BLOOM_ROOT".to_string(),
                type_: "application/vnd.bloom.folder".to_string(),
                parent_id: None,
                size: 0,
                owner_id: msg.aggregate_id,
                metadata,
            };
            let (note, event, _) = eventsourcing::execute(ctx, file::File::new(), &create_cmd)?;

            diesel::insert_into(drive_files::dsl::drive_files)
                .values(&note)
                .execute(ctx)?;
            diesel::insert_into(drive_files_events::dsl::drive_files_events)
                .values(&event)
                .execute(ctx)?;

        }

        // create drive profile
        return Ok(());
    }
}

