use chrono::{DateTime, FixedOffset, NaiveDate, TimeZone};
use serde::{de, Deserialize, Deserializer, Serializer};
const FORMAT: &str = "%Y-%m-%d";

// Custom serialization and deserialization for `NaiveDateTime`
pub mod naive_date_serde {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};
    use super::FORMAT;

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

pub fn deserialize_date<'de, D>(deserializer: D) -> Result<Option<DateTime<FixedOffset>>, D::Error>
where
    D: Deserializer<'de>,
{
    let date_str: Option<String> = Option::deserialize(deserializer)?;

    match date_str {
        Some(date_str) => {
            // Try parsing the string as a NaiveDate (without time component)
            match NaiveDate::parse_from_str(&date_str.trim(), FORMAT) {
                Ok(naive_date) => {
                    // Convert NaiveDate to DateTime<FixedOffset> with a default time (e.g., 00:00:00) and UTC offset
                    let fixed_offset = FixedOffset::east(0); // UTC offset, or you can set your desired offset
                    let datetime = fixed_offset
                        .from_local_date(&naive_date)
                        .unwrap()
                        .and_hms(0, 0, 0);
                    Ok(Some(datetime))
                }
                Err(_) => Err(de::Error::custom("Invalid date format")),
            }
        }
        None => Ok(None), // Handle the case where the date field is missing
    }
}

// Custom serialization function to serialize the DateTime<FixedOffset> as a date string (YYYY-MM-DD)
pub fn serialize_date<S>(
    date: &Option<DateTime<FixedOffset>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match date {
        Some(d) => {
            // Format the DateTime<FixedOffset> as a NaiveDate (YYYY-MM-DD)
            let naive_date = d.date_naive();
            let date_str = naive_date.format(FORMAT).to_string();
            serializer.serialize_str(&date_str)
        }
        None => serializer.serialize_none(),
    }
}
