// Custom serialization and deserialization for `NaiveDateTime`
pub mod naive_date_serde {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d";

    // Serialize as string
    pub fn serialize<S>(datetime: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let formatted = datetime.format(FORMAT).to_string();
        serializer.serialize_str(&formatted)
    }

    // Deserialize from string
    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        NaiveDate::parse_from_str(s, FORMAT).map_err(serde::de::Error::custom)
    }
}


// Custom serialization and deserialization for `NaiveDateTime`
pub mod naive_date_time_serde {
    use chrono::DateTime;
    use chrono::FixedOffset;
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

    pub fn serialize<S>(datetime: &Option<DateTime<FixedOffset>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match datetime {
            Some(dt) => {
                let formatted = dt.format(FORMAT).to_string();
                serializer.serialize_str(&formatted)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<FixedOffset>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<&str> = Option::deserialize(deserializer)?;
        match s {
            Some(s) => DateTime::parse_from_str(s, FORMAT)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}
