use base64::decode;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::de::{Deserialize, Deserializer, Error as DeError};

struct ParsedTimestamp {
    seconds: i64,
    nanoseconds: u32,
}

fn deserialize_timestamp<'de, D>(deserializer: D) -> Result<ParsedTimestamp, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Float(f64),
        Int(i64),
    }

    let number: f64 = match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) => s.parse::<f64>().map_err(DeError::custom)?,
        StringOrNumber::Float(f) => f,
        StringOrNumber::Int(i) => i as f64,
    };

    let number = number.round() as i64;
    let seconds = number / 1000;
    let milliseconds = (number % 1000) as u32;
    let nanoseconds = milliseconds * 1_000_000;

    Ok(ParsedTimestamp {
        seconds,
        nanoseconds,
    })
}

pub(crate) fn deserialize_milliseconds<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp = deserialize_timestamp(deserializer)?;

    Ok(DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(timestamp.seconds, timestamp.nanoseconds),
        Utc,
    ))
}

#[allow(dead_code)]
pub(crate) fn deserialize_seconds<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let timestamp = deserialize_timestamp(deserializer)?;

    Ok(DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(timestamp.seconds, 0),
        Utc,
    ))
}

pub(crate) fn deserialize_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    decode(&s).map_err(DeError::custom)
}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn base64() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_base64")]
            v: Vec<u8>,
        }
        let data = json!({
            "v": "SGVsbG8gV29ybGQ=",
        });
        let decoded: Test = serde_json::from_value(data).unwrap();
        assert_eq!(
            String::from_utf8(decoded.v).unwrap(),
            "Hello World".to_string()
        );
    }
}
