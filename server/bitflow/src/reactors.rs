use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use kernel::{
    KernelError,
    myaccount::domain::account,
};
use crate::{
    domain::{
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
            drive_profiles,
            bitflow_profiles,
            bitflow_profiles_events,
        };
        use diesel::prelude::*;

        if let account::EventData::CreatedV1(ref data) = msg.data {
            let metadata = msg.metadata.clone();

            let drive_profile: drive::domain::Profile = drive_profiles::dsl::drive_profiles
                .filter(drive_profiles::dsl::account_id.eq(data.id))
                .first(ctx)?;

            // create drive profile
            let create_cmd = profile::Create{
                account_id: msg.aggregate_id,
                download_folder_id: drive_profile.home_id,
                metadata: metadata.clone(),
            };
            let (new_profile, event, _) = eventsourcing::execute(ctx, profile::Profile::new(), &create_cmd)?;

            diesel::insert_into(bitflow_profiles::dsl::bitflow_profiles)
                .values(&new_profile)
                .execute(ctx)?;
            diesel::insert_into(bitflow_profiles_events::dsl::bitflow_profiles_events)
                .values(&event)
                .execute(ctx)?;
        }


        return Ok(());
    }
}
