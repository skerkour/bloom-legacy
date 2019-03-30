use image::{FilterType, ImageFormat};
use rusoto_s3::{PutObjectRequest, S3};
use crate::{
    users::domain::user,
    users,
    events::EventMetadata,
    users::validators,
    error::KernelError,
};


#[derive(Clone, Debug)]
pub struct UpdateAvatar {
    pub avatar: Vec<u8>,
    pub s3_bucket: String,
    pub s3_region: String,
    pub metadata: EventMetadata,
}

impl<'a> eventsourcing::Command<'a> for UpdateAvatar {
    type Aggregate = user::User;
    type Event = user::Event;
    type Context = rusoto_s3::S3Client;
    type Error = KernelError;
    type NonStoredData = ();

    fn validate(&self, _ctx: &Self::Context, _aggregate: &Self::Aggregate) -> Result<(), Self::Error> {
        if self.avatar.len() > users::AVATAR_MAX_SIZE {
            return Err(KernelError::Validation("Image size must be inferior or equal to 3MB.".to_string()));
        }

        let content_type = mimesniff::detect_content_type(&self.avatar);
        if content_type != mime::IMAGE_PNG && content_type != mime::IMAGE_JPEG {
            return Err(KernelError::Validation("Image format must be png, jpg or jpeg.".to_string()));
        }

        return Ok(());
    }


    fn build_event(&self, ctx: &Self::Context, aggregate: &Self::Aggregate) -> Result<(Self::Event, Self::NonStoredData), Self::Error> {


        // resize image to user::AVATAR_RESIZE
        let img = image::load_from_memory(&self.avatar)?;
        let scaled = img.resize(users::AVATAR_RESIZE as u32, users::AVATAR_RESIZE as u32, FilterType::Lanczos3);
        let mut result = Vec::new();
        // encode to jpeg
        scaled.write_to(&mut result, ImageFormat::JPEG)?;

        // upload to s3
        let id = uuid::Uuid::new_v4();
        let key = format!("user/avatars/{}.jpeg", id);
        let avatar_url = format!("https://s3.{}.amazonaws.com/{}/{}", &self.s3_region, &self.s3_bucket, &key);
        let req = PutObjectRequest {
            bucket: self.s3_bucket.clone(),
            key: key,
            body: Some(result.into()),
            ..Default::default()
        };
        let result = ctx.put_object(req).sync().expect("Couldn't PUT object");

        let event_data = user::EventData::AvatarUpdatedV1(user::AvatarUpdatedV1{
            avatar_url,
        });

        return  Ok((user::Event{
            id: uuid::Uuid::new_v4(),
            timestamp: chrono::Utc::now(),
            data: event_data,
            aggregate_id: aggregate.id,
            metadata: self.metadata.clone(),
        }, ()));
    }
}
