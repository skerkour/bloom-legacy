use crate::{domain, domain::file};
use actix::{Handler, Message};
use kernel::{db::DbActor, KernelError};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Move {
    pub to: uuid::Uuid,
    pub files: Vec<uuid::Uuid>,
    pub owner_id: uuid::Uuid,
    pub request_id: uuid::Uuid,
    pub session_id: uuid::Uuid,
}

impl Message for Move {
    type Result = Result<(), KernelError>;
}

impl Handler<Move> for DbActor {
    type Result = Result<(), KernelError>;

    fn handle(&mut self, msg: Move, _: &mut Self::Context) -> Self::Result {
        use diesel::prelude::*;
        use kernel::db::schema::drive_files;

        let conn = self.pool.get().map_err(|_| KernelError::R2d2)?;

        return Ok(conn.transaction::<_, KernelError, _>(|| {
            for file_id in msg.files.into_iter() {
                let file_to_move: domain::File = drive_files::dsl::drive_files
                    .filter(drive_files::dsl::id.eq(file_id))
                    .filter(drive_files::dsl::owner_id.eq(msg.owner_id))
                    .filter(drive_files::dsl::trashed_at.is_null())
                    .first(&conn)?;

                let move_cmd = file::Move { to: msg.to };
                let (file_to_move, _) = eventsourcing::execute(&conn, file_to_move, &move_cmd)?;
                diesel::update(&file_to_move)
                    .set(&file_to_move)
                    .execute(&conn)?;
            }

            return Ok(());
        })?);
    }
}
