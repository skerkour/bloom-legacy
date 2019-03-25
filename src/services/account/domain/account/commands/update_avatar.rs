use crate::{
    services::account::domain::account,
    services::account as account_service,
    services::common::events::EventMetadata,
    services::account::validators,
    error::KernelError,
};
use diesel::{
    PgConnection,
    r2d2::{PooledConnection, ConnectionManager},
};
use image::{FilterType, ImageFormat};


#[derive(Clone, Debug)]
pub struct UpdateAvatar {
    pub avatar: Vec<u8>,
    pub metadata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for UpdateAvatar {
    type Aggregate = account::Account;
    type Event = account::Event;
    type Context = PooledConnection<ConnectionManager<PgConnection>>;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        if self.avatar.len() > account_service::AVATAR_MAX_SIZE {
            return Err(KernelError::Validation("Image size must be inferior or equal to 3MB.".to_string()));
        }

        let content_type = mimesniff::detect_content_type(&self.avatar);
        if content_type != mime::IMAGE_PNG && content_type != mime::IMAGE_JPEG {
            return Err(KernelError::Validation("Image format must be png, jpg or jpeg.".to_string()));
        }

        return Ok(());
    }


    fn build_event(&self, _ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {
        let data = account::EventData::AvatarUpdatedV1(account::AvatarUpdatedV1{
            avatar_url: aggregate.avatar_url.clone(),
        });

        // resize image to account::AVATAR_RESIZE
        let img = image::load_from_memory(&self.avatar)?;
        let scaled = img.resize(account_service::AVATAR_RESIZE as u32, account_service::AVATAR_RESIZE as u32, FilterType::Lanczos3);
        let mut result = Vec::new();
        // encode to jpeg
        scaled.write_to(&mut result, ImageFormat::JPEG)?;

        // TODO: uplaod to s3

        // let req = PutObjectRequest {
        //         bucket: bucket.to_owned(),
        //         key: dest_filename.to_owned(),
        //         body: Some(contents.into()),
        //         ..Default::default()
        //     };
        // let result = client.put_object(req).sync().expect("Couldn't PUT object");

        return  Ok((account::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
