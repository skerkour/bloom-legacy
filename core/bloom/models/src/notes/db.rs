use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Note {
    pub id: String,
    #[serde(with = "dart_date_format")]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "dart_date_format")]
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(with = "dart_date_format_opt")]
    pub archived_at: Option<chrono::DateTime<chrono::Utc>>,
    pub title: String,
    pub body: String,
    pub color: i64,
    pub is_pinned: bool,
}

/// https://serde.rs/custom-date-format.html
mod dart_date_format {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let date = DateTime::parse_from_rfc3339(&s).map_err(serde::de::Error::custom)?;
        return Ok(date.into());
    }
}

mod dart_date_format_opt {
    use chrono::{DateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => {
                let s = date.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let date: Option<DateTime<Utc>> = Option::deserialize(deserializer)?;
        return Ok(date);
    }
}
