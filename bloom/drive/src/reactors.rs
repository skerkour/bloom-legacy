use crate::{
    controllers,
    domain::{file, profile},
    BLOOM_ROOT_NAME, DEFAULT_FOLDERS, FOLDER_TYPE,
};
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
        use kernel::db::schema::{drive_files, drive_profiles};

        // create home
        let create_cmd = file::Create {
            name: BLOOM_ROOT_NAME.to_string(),
            type_: FOLDER_TYPE.to_string(),
            parent_id: None,
            size: 0,
            owner_id: event.id,
        };
        let (home, _) = eventsourcing::execute(ctx, file::File::new(), &create_cmd)?;
        diesel::insert_into(drive_files::dsl::drive_files)
            .values(&home)
            .execute(ctx)?;

        // create drive profile
        let create_cmd = profile::Create {
            account_id: event.id,
            home_id: home.id,
        };
        let (new_profile, _) = eventsourcing::execute(ctx, profile::Profile::new(), &create_cmd)?;

        diesel::insert_into(drive_profiles::dsl::drive_profiles)
            .values(&new_profile)
            .execute(ctx)?;

        // create all default folders
        for default_folder in DEFAULT_FOLDERS.iter() {
            let create_cmd = file::Create {
                name: default_folder.to_string(),
                type_: FOLDER_TYPE.to_string(),
                parent_id: Some(home.id),
                size: 0,
                owner_id: event.id,
            };
            let (new_folder, _) = eventsourcing::execute(ctx, file::File::new(), &create_cmd)?;

            diesel::insert_into(drive_files::dsl::drive_files)
                .values(&new_folder)
                .execute(ctx)?;
        }

        return Ok(());
    }
}

pub struct AccountDeleted;
impl eventsourcing::Subscription for AccountDeleted {
    type Error = KernelError;
    type Event = account::Deleted;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;

    fn handle(&self, ctx: &Self::Context, event: &Self::Event) -> Result<(), Self::Error> {
        use diesel::prelude::*;
        use kernel::db::schema::drive_files;

        // find home
        let home_file: file::File = drive_files::dsl::drive_files
            .filter(drive_files::dsl::name.eq(crate::BLOOM_ROOT_NAME))
            .filter(drive_files::dsl::owner_id.eq(event.id))
            .first(ctx)?;

        // delete home
        controllers::delete_file_with_children(
            home_file,
            event.s3_client.clone(),
            event.s3_bucket.clone(),
            ctx,
        )?;

        return Ok(());
    }
}
