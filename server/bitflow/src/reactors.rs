use crate::domain::profile;
use diesel::{
    r2d2::{ConnectionManager, PooledConnection},
    PgConnection,
};
use kernel::{myaccount::domain::account, KernelError};

pub struct AccountCreated;
impl eventsourcing::Subscription for AccountCreated {
    type Error = KernelError;
    type Event = account::Created;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;

    fn handle(&self, ctx: &Self::Context, event: &Self::Event) -> Result<(), Self::Error> {
        use diesel::prelude::*;
        use kernel::db::schema::{bitflow_profiles, drive_profiles};

        let drive_profile: drive::domain::Profile = drive_profiles::dsl::drive_profiles
            .filter(drive_profiles::dsl::account_id.eq(event.id))
            .first(ctx)?;

        // create drive profile
        let create_cmd = profile::Create {
            account_id: event.id,
            download_folder_id: drive_profile.home_id,
        };
        let (new_profile, _) = eventsourcing::execute(ctx, profile::Profile::new(), &create_cmd)?;

        diesel::insert_into(bitflow_profiles::dsl::bitflow_profiles)
            .values(&new_profile)
            .execute(ctx)?;

        return Ok(());
    }
}
