use super::Repository;
use crate::{entities::Conversation, Error};
use kernel::db::Queryer;
use stdx::{log::error, sqlx};

impl Repository {
    pub async fn update_conversation<'c, C: Queryer<'c>>(
        &self,
        db: C,
        conversation: &Conversation,
    ) -> Result<(), Error> {
        const QUERY: &str = "UPDATE inbox_conversations SET
        updated_at = $1, archived_at = $2, trashed_at = $3, last_message_at = $4, is_spam = $5,
        name = $6, description = $7, anonymous_id = $8
        WHERE id = $9";

        match sqlx::query(QUERY)
            .bind(conversation.updated_at)
            .bind(conversation.archived_at)
            .bind(conversation.trashed_at)
            .bind(conversation.last_message_at)
            .bind(conversation.is_spam)
            .bind(&conversation.name)
            .bind(&conversation.description)
            .bind(conversation.anonymous_id)
            .bind(conversation.id)
            .execute(db)
            .await
        {
            Err(err) => {
                error!("files.update_conversation: Updating conversation: {}", &err);
                Err(err.into())
            }
            Ok(_) => Ok(()),
        }
    }
}
