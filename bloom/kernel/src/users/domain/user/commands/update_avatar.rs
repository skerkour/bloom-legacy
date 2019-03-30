use crate::{
    services::account::domain::account,
    services::account as account_service,
    services::common::events::EventMetadata,
    services::account::validators,
    error::KernelError,
};
use image::{FilterType, ImageFormat};
use rusoto_s3::{PutObjectRequest, S3};


#[derive(Clone, Debug)]
pub struct UpdateAvatar {
    pub avatar: Vec<u8>,
    pub s3_bucket: String,
    pub s3_region: String,
    pub metadata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for UpdateAvatar {
    type Aggregate = account::Account;
    type Event = account::Event;
    type Context = rusoto_s3::S3Client;
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


    fn build_event(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {


        // resize image to account::AVATAR_RESIZE
        let img = image::load_from_memory(&self.avatar)?;
        let scaled = img.resize(account_service::AVATAR_RESIZE as u32, account_service::AVATAR_RESIZE as u32, FilterType::Lanczos3);
        let mut result = Vec::new();
        // encode to jpeg
        scaled.write_to(&mut result, ImageFormat::JPEG)?;

        // upload to s3
        let id = uuid::Uuid::new_v4();
        let key = format!("account/avatars/{}.jpeg", id);
        let avatar_url = format!("https://s3.{}.amazonaws.com/{}/{}", &self.s3_region, &self.s3_bucket, &key);
        let req = PutObjectRequest {
            bucket: self.s3_bucket.clone(),
            key: key,
            body: Some(result.into()),
            ..Default::default()
        };
        let result = ctx.put_object(req).sync().expect("Couldn't PUT object");

        let event_data = account::EventData::AvatarUpdatedV1(account::AvatarUpdatedV1{
            avatar_url,
        });

        return  Ok((account::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: event_data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
