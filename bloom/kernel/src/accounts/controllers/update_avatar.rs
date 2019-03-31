use actix::{Message, Handler};
use crate::{
    db::DbActor,
    accounts::domain::{
        Account,
        account,
    },
    events::EventMetadata,
};
use crate::error::KernelError;


#[derive(Clone)]
pub struct UpdateAvatar {
    pub account: Account,
    pub avatar: Vec<u8>,
    pub s3_bucket: String,
    pub s3_region: String,
    pub s3_client: rusoto_s3::S3Client,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for UpdateAvatar {
    type Result = Result<Account, KernelError>;
}

impl Handler<UpdateAvatar> for DbActor {
    type Result = Result<Account, KernelError>;

    fn handle(&mut self, msg: UpdateAvatar, _: &mut Self::Context) -> Self::Result {
        use crate::db::schema::{
            kernel_accounts_events,
        };
        use diesel::prelude::*;


        let conn = self.pool.get()
            .map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let metadata = EventMetadata{
                actor_id: Some(msg.account.id),
                request_id: Some(msg.request_id),
                session_id: Some(msg.session_id),
            };

            let account_to_update = msg.account;

            let update_first_name_cmd = account::UpdateAvatar{
                avatar: msg.avatar,
                s3_bucket: msg.s3_bucket,
                s3_region: msg.s3_region,
                metadata: metadata.clone(),
            };

            let (account_to_update, event, _) = eventsourcing::execute(&msg.s3_client, account_to_update, &update_first_name_cmd)?;

            // update account
            diesel::update(&account_to_update)
                .set(&account_to_update)
                .execute(&conn)?;
            diesel::insert_into(kernel_accounts_events::dsl::kernel_accounts_events)
                .values(&event)
                .execute(&conn)?;

            return Ok(account_to_update);
        })?);
    }
}
