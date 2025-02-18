use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeZone};
use serde::{de, Deserialize, Deserializer, Serializer};
const FORMAT: &str = "%Y-%m-%d";

// Custom serialization and deserialization for `NaiveDateTime`
pub mod naive_date_serde {
    use super::FORMAT;
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer, Serializer};

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
            // 解析为 NaiveDate
            match NaiveDate::parse_from_str(&date_str.trim(), FORMAT) {
                Ok(naive_date) => {
                    // 使用 UTC 偏移（或者根据需求设置其他偏移量）
                    let fixed_offset = FixedOffset::east_opt(0)
                        .ok_or_else(|| de::Error::custom("Invalid UTC offset"))?; // 处理 Option

                    // 构建 NaiveDateTime（默认时间为 00:00:00）
                    let naive_datetime = NaiveDateTime::new(
                        naive_date,
                        NaiveTime::from_hms_opt(0, 0, 0)
                            .ok_or_else(|| de::Error::custom("Invalid native time"))?,
                    );

                    // 转换为 DateTime<FixedOffset>
                    let datetime = fixed_offset
                        .from_local_datetime(&naive_datetime)
                        .single() // 获取唯一的 Option<DateTime<FixedOffset>>
                        .ok_or_else(|| de::Error::custom("Failed to convert to DateTime"))?;

                    Ok(Some(datetime))
                }
                Err(_) => Err(de::Error::custom("Invalid date format")), // 处理解析错误
            }
        }
        None => Ok(None), // 处理字段为空的情况
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
