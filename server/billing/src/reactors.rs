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
            billing_profiles,
            billing_profiles_events,
        };
        use diesel::prelude::*;

        if let account::EventData::CreatedV1(ref _data) = msg.data {
            let metadata = msg.metadata.clone();

            // create drive profile
            let create_cmd = profile::Create{
                account_id: msg.aggregate_id,
                metadata: metadata.clone(),
            };
            let (new_profile, event, _) = eventsourcing::execute(ctx, profile::Profile::new(), &create_cmd)?;

            diesel::insert_into(billing_profiles::dsl::billing_profiles)
                .values(&new_profile)
                .execute(ctx)?;
            diesel::insert_into(billing_profiles_events::dsl::billing_profiles_events)
                .values(&event)
                .execute(ctx)?;
        }
        return Ok(());
    }
}
