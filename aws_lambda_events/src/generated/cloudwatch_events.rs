use chrono::{DateTime, Utc};
use custom_serde::*;
use serde_json::Value;

/// `CloudWatchEvent` is the outer structure of an event sent via CloudWatch Events.
/// For examples of events that come via CloudWatch Events, see https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/EventTypes.html
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CloudWatchEvent {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub id: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "detail-type")]
    pub detail_type: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "detail-type")]
    pub detail_type: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub source: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub source: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "account")]
    pub account_id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "account")]
    pub account_id: String,
    pub time: DateTime<Utc>,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: String,
    pub resources: Vec<String>,
    pub detail: Value,
}
