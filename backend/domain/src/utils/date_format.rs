// Required for serializing/deserializing DateTimeUtcForm
pub mod date_format {
    use crate::utils::date_time_utc_form::DateTimeUtcForm;
    use chrono::{DateTime, NaiveDateTime, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%d/%m/%Y %H:%M";

    pub fn serialize<S>(date: &DateTimeUtcForm, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let date = &date.inner();
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTimeUtcForm, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = NaiveDateTime::parse_from_str(&s, FORMAT).map_err(serde::de::Error::custom)?;
        Ok(DateTimeUtcForm {
            time_taken: DateTime::<Utc>::from_naive_utc_and_offset(dt, Utc),
        })
    }
}
