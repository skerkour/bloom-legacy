use crate::error::KernelError;
use crate::{
    db::DbActor,
    accounts::domain::{account, Account},
};
use actix::{Handler, Message};

#[derive(Clone)]
pub struct UpdateAvatar {
    pub account: Account,
    pub avatar: Vec<u8>,
    pub s3_bucket: String,
    pub s3_base_url: String,
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
        use diesel::prelude::*;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            let account_to_update = msg.account;

            let update_first_name_cmd = account::UpdateAvatar {
                avatar: msg.avatar,
                s3_bucket: msg.s3_bucket,
                s3_base_url: msg.s3_base_url,
            };

            let (account_to_update, _) =
                eventsourcing::execute(&msg.s3_client, account_to_update, &update_first_name_cmd)?;

            // update account
            diesel::update(&account_to_update)
                .set(&account_to_update)
                .execute(&conn)?;

            return Ok(account_to_update);
        })?);
    }
}
