use chrono::{DateTime, Utc};
use custom_serde::*;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::Value;

/// `CloudWatchEvent` is the outer structure of an event sent via CloudWatch Events.
/// For examples of events that come via CloudWatch Events, see https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/EventTypes.html
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CloudWatchEvent<T1=Value>
where T1: DeserializeOwned,
      T1: Serialize,
{
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "detail-type")]
    pub detail_type: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub source: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "account")]
    pub account_id: Option<String>,
    pub time: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    pub resources: Vec<String>,
    #[serde(bound="")]
    pub detail: T1,
}
