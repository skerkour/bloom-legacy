use crate::{error::KernelError, accounts, accounts::domain::account};
use eventsourcing::{Event, EventTs};
use image::{FilterType, ImageFormat};
use rusoto_s3::{PutObjectRequest, S3};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug)]
pub struct UpdateAvatar {
    pub avatar: Vec<u8>,
    pub s3_bucket: String,
    pub s3_base_url: String,
}

impl eventsourcing::Command for UpdateAvatar {
    type Aggregate = account::Account;
    type Event = AvatarUpdated;
    type Context = rusoto_s3::S3Client;
    type Error = KernelError;

    fn validate(
        &self,
        _ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<(), Self::Error> {
        if self.avatar.len() > accounts::AVATAR_MAX_SIZE {
            return Err(KernelError::Validation(
                "Image size must be inferior or equal to 3MB.".to_string(),
            ));
        }

        let content_type = mimesniff::detect_content_type(&self.avatar);
        if content_type != mime::IMAGE_PNG && content_type != mime::IMAGE_JPEG {
            return Err(KernelError::Validation(
                "Image format must be png, jpg or jpeg.".to_string(),
            ));
        }

        return Ok(());
    }

    fn build_event(
        &self,
        ctx: &Self::Context,
        _aggregate: &Self::Aggregate,
    ) -> Result<Self::Event, Self::Error> {
        // resize image
        let img = image::load_from_memory(&self.avatar)?;
        let scaled = img.resize(
            accounts::AVATAR_RESIZE as u32,
            accounts::AVATAR_RESIZE as u32,
            FilterType::Lanczos3,
        );
        let mut result = Vec::new();
        // encode to jpeg
        scaled.write_to(&mut result, ImageFormat::JPEG)?;

        // upload to s3
        let id = uuid::Uuid::new_v4();
        let key = format!("account/avatars/{}.jpeg", id);
        let avatar_url = format!("{}/{}/{}", &self.s3_base_url, &self.s3_bucket, &key);
        let req = PutObjectRequest {
            bucket: self.s3_bucket.clone(),
            key,
            body: Some(result.into()),
            ..Default::default()
        };
        // TODO: handle error
        ctx.put_object(req).sync().expect("Couldn't PUT object");

        return Ok(AvatarUpdated {
            timestamp: chrono::Utc::now(),
            avatar_url,
        });
    }
}

// Event
#[derive(Clone, Debug, Deserialize, EventTs, Serialize)]
pub struct AvatarUpdated {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub avatar_url: String,
}

impl Event for AvatarUpdated {
    type Aggregate = account::Account;

    fn apply(&self, aggregate: Self::Aggregate) -> Self::Aggregate {
        return Self::Aggregate {
            avatar_url: self.avatar_url.clone(),
            ..aggregate
        };
    }
}
