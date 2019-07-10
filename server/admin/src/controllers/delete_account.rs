use actix::{Handler, Message};
use kernel::{
    db::DbActor,
    error::KernelError,
    myaccount::domain::{account, Account, DeletedUsername},
};

#[derive(Clone)]
pub struct DeleteAccount {
    pub actor: Account,
    pub s3_bucket: String,
    pub s3_client: rusoto_s3::S3Client,
    pub account_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
}

impl Message for DeleteAccount {
    type Result = Result<(), KernelError>;
}

impl Handler<DeleteAccount> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: DeleteAccount, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::{kernel_accounts, kernel_deleted_usernames};

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            if !msg.actor.is_admin {
                return Err(KernelError::Forbidden("Admin role is required".to_string()));
            }

            let account_to_delete: Account = kernel_accounts::dsl::kernel_accounts
                .filter(kernel_accounts::dsl::id.eq(msg.account_id))
                .for_update()
                .first(&conn)?;

            let delete_cmd = account::Delete {
                actor: msg.actor,
                s3_client: msg.s3_client,
                s3_bucket: msg.s3_bucket,
            };
            let (account_to_delete, _) =
                eventsourcing::execute(&conn, account_to_delete, &delete_cmd)?;
            diesel::delete(&account_to_delete).execute(&conn)?;

            // save username to not use it again
            let username = DeletedUsername {
                username: account_to_delete.username.clone(),
            };
            diesel::insert_into(kernel_deleted_usernames::dsl::kernel_deleted_usernames)
                .values(&username)
                .execute(&conn)?;

            return Ok(());
        })?);
    }
}
