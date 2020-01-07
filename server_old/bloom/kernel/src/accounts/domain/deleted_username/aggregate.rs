use crate::db::schema::deleted_usernames;
use diesel::Queryable;
use serde::{Deserialize, Serialize};

#[derive(AsChangeset, Clone, Debug, Deserialize, Insertable, Queryable, Serialize)]
#[table_name = "deleted_usernames"]
#[changeset_options(treat_none_as_null = "true")]
pub struct DeletedUsername {
    pub username: String,
}

impl DeletedUsername {
    // create a new, unitialized DeletedUsername
    pub fn new() -> Self {
        return DeletedUsername {
            username: String::new(),
        };
    }
}

impl Default for DeletedUsername {
    fn default() -> Self {
        Self::new()
    }
}
