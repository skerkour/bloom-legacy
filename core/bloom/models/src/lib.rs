pub mod calendar;
pub mod notes;

/// https://serde.rs/custom-date-format.html
/// we need to do this because of Dart DateTime.parse which only support up to
/// 6 digits second fraction
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
