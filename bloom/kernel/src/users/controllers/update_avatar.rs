use actix::{Message, Handler};
use crate::{
    db::DbActor,
    users::domain::{
        User,
        user,
    },
    events::EventMetadata,
};
use crate::error::KernelError;


#[derive(Clone)]
pub struct UpdateAvatar {
    pub user: User,
    pub avatar: Vec<u8>,
    pub s3_bucket: String,
    pub s3_region: String,
    pub s3_client: rusoto_s3::S3Client,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for UpdateAvatar {
    type Result = Result<User, KernelError>;
}

impl Handler<UpdateAvatar> for DbActor {
    type Result = Result<User, KernelError>;

    fn handle(&mut self, msg: UpdateAvatar, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_users_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: Some(msg.user.id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            let user_to_update = msg.user;

            let update_first_name_cmd = user::UpdateAvatar{
                avatar: msg.avatar,
                s3_bucket: msg.s3_bucket,
                s3_region: msg.s3_region,
                metadata: metadata.clone(),
            };

            let (user_to_update, event, _) = eventsourcing::execute(&msg.s3_client, user_to_update, &update_first_name_cmd)?;

            // update user
            diesel::update(&user_to_update)
                .set(&user_to_update)
                .execute(&conn)?;
            diesel::insert_into(kernel_users_events::dsl::kernel_users_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(user_to_update);
        })?);
    }
}
