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
        file,
        profile,
    },
    DEFAULT_FOLDERS,
    BLOOM_ROOT_NAME,
    FOLDER_TYPE,
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
            drive_profiles,
            drive_profiles_events,
        };
        use diesel::prelude::*;

        if let account::EventData::CreatedV1(ref _data) = msg.data {
            let metadata = msg.metadata.clone();

            // create home
            let create_cmd = file::Create{
                name: BLOOM_ROOT_NAME.to_string(),
                type_: FOLDER_TYPE.to_string(),
                parent_id: None,
                size: 0,
                owner_id: msg.aggregate_id,
                metadata: metadata.clone(),
            };
            let (home, event, _) = eventsourcing::execute(ctx, file::File::new(), &create_cmd)?;
            diesel::insert_into(drive_files::dsl::drive_files)
                .values(&home)
                .execute(ctx)?;
            diesel::insert_into(drive_files_events::dsl::drive_files_events)
                .values(&event)
                .execute(ctx)?;


            // create drive profile
            let create_cmd = profile::Create{
                account_id: msg.aggregate_id,
                home_id: home.id,
                metadata: metadata.clone(),
            };
            let (new_profile, event, _) = eventsourcing::execute(ctx, profile::Profile::new(), &create_cmd)?;

            diesel::insert_into(drive_profiles::dsl::drive_profiles)
                .values(&new_profile)
                .execute(ctx)?;
            diesel::insert_into(drive_profiles_events::dsl::drive_profiles_events)
                .values(&event)
                .execute(ctx)?;

            // create all default folders
            for default_folder in DEFAULT_FOLDERS.iter() {
                let create_cmd = file::Create{
                    name: default_folder.to_string(),
                    type_: FOLDER_TYPE.to_string(),
                    parent_id: Some(home.id),
                    size: 0,
                    owner_id: msg.aggregate_id,
                    metadata: metadata.clone(),
                };
                let (new_folder, event, _) = eventsourcing::execute(ctx, file::File::new(), &create_cmd)?;

                diesel::insert_into(drive_files::dsl::drive_files)
                    .values(&new_folder)
                    .execute(ctx)?;
                diesel::insert_into(drive_files_events::dsl::drive_files_events)
                    .values(&event)
                    .execute(ctx)?;
            }

        }


        return Ok(());
    }
}
