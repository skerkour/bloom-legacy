#![cfg_attr(feature = "cargo-clippy", allow(clippy::all))]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Uuid as UuidDiesel;
use std::io::Write;
use uuid;

#[derive(Clone, Debug, AsExpression, PartialEq, FromSqlRow, Serialize, Deserialize, Hash, Eq)]
#[sql_type = "UuidDiesel"]
pub struct Uuid(pub uuid::Uuid);

impl ToSql<UuidDiesel, Pg> for Uuid {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        out.write_all(self.0.as_bytes())?;
        Ok(IsNull::No)
    }
}

impl FromSql<UuidDiesel, Pg> for Uuid {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let bytes = not_none!(bytes);
        Ok(Uuid(uuid::Uuid::from_slice(bytes)?))
    }
}

impl From<uuid::Uuid> for Uuid {
    fn from(uuid: uuid::Uuid) -> Self {
        Uuid(uuid)
    }
}
