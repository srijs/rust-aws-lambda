use base64::{decode, encode};
use chrono::{DateTime, TimeZone, Utc};
use serde;
use serde::de::{Deserialize, Deserializer, Error as DeError};
use serde::ser::Serializer;
use std;
use std::collections::HashMap;

fn normalize_timestamp<'de, D>(deserializer: D) -> Result<(u64, u64), D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNumber {
        String(String),
        Float(f64),
        Int(u64),
    }

    let input: f64 = match StringOrNumber::deserialize(deserializer)? {
        StringOrNumber::String(s) => s.parse::<f64>().map_err(DeError::custom)?,
        StringOrNumber::Float(f) => f,
        StringOrNumber::Int(i) => i as f64,
    };

    // We need to do this due to floating point issues.
    let input_as_string = format!("{}", input);
    let parts: Result<Vec<u64>, _> = input_as_string
        .split('.')
        .map(|x| x.parse::<u64>().map_err(DeError::custom))
        .collect();
    let parts = parts?;
    if parts.len() > 1 {
        Ok((parts[0], parts[1]))
    } else {
        Ok((parts[0], 0))
    }
}

pub(crate) fn serialize_milliseconds<S>(
    date: &DateTime<Utc>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ts_with_millis = date.timestamp() * 1000
        + date.timestamp_subsec_millis() as i64 * 10
        + date.timestamp_subsec_nanos() as i64;

    serializer.serialize_str(&ts_with_millis.to_string())
}

pub(crate) fn deserialize_milliseconds<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let (whole, frac) = normalize_timestamp(deserializer)?;
    assert_eq!(frac, 0);
    let seconds: f64 = (whole / 1000) as f64;
    let milliseconds: u32 = (seconds.fract() * 1000f64) as u32;
    let nanos = milliseconds * 1_000_000;
    Ok(Utc.timestamp(seconds as i64, nanos as u32))
}

pub(crate) fn serialize_seconds<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let seconds = date.timestamp();
    let milliseconds = date.timestamp_subsec_millis();
    let combined = format!("{}.{}", seconds, milliseconds);
    if milliseconds > 0 {
        serializer.serialize_str(&combined)
    } else {
        serializer.serialize_str(&seconds.to_string())
    }
}

#[allow(dead_code)]
pub(crate) fn deserialize_seconds<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let (whole, frac) = normalize_timestamp(deserializer)?;
    let seconds = whole;
    let nanos = frac * 1_000_000;
    Ok(Utc.timestamp(seconds as i64, nanos as u32))
}

pub(crate) fn deserialize_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    decode(&s).map_err(DeError::custom)
}

pub(crate) fn serialize_base64<S>(value: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&encode(value))
}

/// Deserializes `Option<String>`, mapping JSON `null` or the empty string `""` to `None`.
#[cfg(not(feature = "string-null-empty"))]
pub(crate) fn deserialize_lambda_string<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    match Option::deserialize(deserializer)? {
        Some(s) => {
            if s == "" {
                Ok(None)
            } else {
                Ok(Some(s))
            }
        }
        None => Ok(None),
    }
}

/// Deserializes `String`, mapping JSON `null` to the empty string `""`.
#[cfg(feature = "string-null-empty")]
pub(crate) fn deserialize_lambda_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let result: String = match Option::deserialize(deserializer)? {
        Some(s) => s,
        None => String::default(),
    };
    Ok(result)
}

/// Deserializes `HashMap<_>`, mapping JSON `null` to an empty map.
pub(crate) fn deserialize_lambda_map<'de, D, K, V>(
    deserializer: D,
) -> Result<HashMap<K, V>, D::Error>
where
    D: Deserializer<'de>,
    K: serde::Deserialize<'de>,
    K: std::hash::Hash,
    K: std::cmp::Eq,
    V: serde::Deserialize<'de>,
{
    // https://github.com/serde-rs/serde/issues/1098
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or(HashMap::default()))
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::TimeZone;
    use serde_json;

    #[test]
    fn test_deserialize_base64() {
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

    #[test]
    fn test_serialize_base64() {
        #[derive(Serialize)]
        struct Test {
            #[serde(serialize_with = "serialize_base64")]
            v: Vec<u8>,
        }
        let instance = Test {
            v: "Hello World".as_bytes().to_vec(),
        };
        let encoded = serde_json::to_string(&instance).unwrap();
        assert_eq!(encoded, r#"{"v":"SGVsbG8gV29ybGQ="}"#.to_string());
    }

    #[test]
    fn test_deserialize_milliseconds() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_milliseconds")]
            v: DateTime<Utc>,
        }
        let expected = Utc.ymd(2017, 10, 05).and_hms_nano(15, 33, 44, 0);

        // Test parsing strings.
        let data = json!({
                "v": "1507217624302",
            });
        let decoded: Test = serde_json::from_value(data).unwrap();
        assert_eq!(expected, decoded.v,);
        // Test parsing ints.
        let decoded: Test = serde_json::from_slice(r#"{"v":1507217624302}"#.as_bytes()).unwrap();
        assert_eq!(expected, decoded.v,);
        // Test parsing floats.
        let data = json!({
                "v": 1507217624302.0,
            });
        let decoded: Test = serde_json::from_value(data).unwrap();
        assert_eq!(expected, decoded.v,);
    }

    #[test]
    fn test_serialize_milliseconds() {
        #[derive(Serialize)]
        struct Test {
            #[serde(serialize_with = "serialize_milliseconds")]
            v: DateTime<Utc>,
        }
        let instance = Test {
            v: Utc.ymd(1983, 7, 22).and_hms_nano(1, 0, 0, 99),
        };
        let encoded = serde_json::to_string(&instance).unwrap();
        assert_eq!(encoded, String::from(r#"{"v":"427683600099"}"#));
    }

    #[test]
    fn test_serialize_seconds() {
        #[derive(Serialize)]
        struct Test {
            #[serde(serialize_with = "serialize_seconds")]
            v: DateTime<Utc>,
        }

        // Make sure nanoseconds are chopped off.
        let instance = Test {
            v: Utc.ymd(1983, 7, 22).and_hms_nano(1, 0, 0, 99),
        };
        let encoded = serde_json::to_string(&instance).unwrap();
        assert_eq!(encoded, String::from(r#"{"v":"427683600"}"#));

        // Make sure milliseconds are included.
        let instance = Test {
            v: Utc.ymd(1983, 7, 22).and_hms_nano(1, 0, 0, 2_000_000),
        };
        let encoded = serde_json::to_string(&instance).unwrap();
        assert_eq!(encoded, String::from(r#"{"v":"427683600.2"}"#));

        // Make sure milliseconds are included.
        let instance = Test {
            v: Utc.ymd(1983, 7, 22).and_hms_nano(1, 0, 0, 1234_000_000),
        };
        let encoded = serde_json::to_string(&instance).unwrap();
        assert_eq!(encoded, String::from(r#"{"v":"427683600.1234"}"#));
    }

    #[cfg(feature = "string-null-empty")]
    #[test]
    fn test_deserialize_string() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_lambda_string")]
            v: String,
        }
        let input = json!({
          "v": "",
        });
        let decoded: Test = serde_json::from_value(input).unwrap();
        assert_eq!("".to_string(), decoded.v);

        let input = json!({
          "v": null,
        });
        let decoded: Test = serde_json::from_value(input).unwrap();
        assert_eq!("".to_string(), decoded.v);
    }

    #[cfg(feature = "string-null-none")]
    #[test]
    fn test_deserialize_string() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_lambda_string")]
            v: Option<String>,
        }
        let input = json!({
          "v": "",
        });
        let decoded: Test = serde_json::from_value(input).unwrap();
        assert_eq!(None, decoded.v);
        let input = json!({
          "v": null,
        });
        let decoded: Test = serde_json::from_value(input).unwrap();
        assert_eq!(None, decoded.v);
        let input = json!({
          "v": "foo",
        });
        let decoded: Test = serde_json::from_value(input).unwrap();
        assert_eq!(Some("foo".to_string()), decoded.v);
    }

    #[test]
    fn test_deserialize_map() {
        #[derive(Deserialize)]
        struct Test {
            #[serde(deserialize_with = "deserialize_lambda_map")]
            v: HashMap<String, String>,
        }
        let input = json!({
          "v": {},
        });
        let decoded: Test = serde_json::from_value(input).unwrap();
        assert_eq!(HashMap::new(), decoded.v);

        let input = json!({
          "v": null,
        });
        let decoded: Test = serde_json::from_value(input).unwrap();
        assert_eq!(HashMap::new(), decoded.v);
    }
}
