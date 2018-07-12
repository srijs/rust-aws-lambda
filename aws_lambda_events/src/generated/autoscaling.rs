use chrono::{DateTime, Utc};
use custom_serde::*;
use serde_json::Value;
use std::collections::HashMap;

/// `AutoScalingEvent` struct is used to parse the json for auto scaling event types //
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AutoScalingEvent {
    /// The version of event data
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    /// The version of event data
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: String,
    /// The unique ID of the event
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub id: Option<String>,
    /// The unique ID of the event
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub id: String,
    /// Details about event type
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "detail-type")]
    pub detail_type: Option<String>,
    /// Details about event type
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "detail-type")]
    pub detail_type: String,
    /// Source of the event
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub source: Option<String>,
    /// Source of the event
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub source: String,
    /// AccountId
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "account")]
    pub account_id: Option<String>,
    /// AccountId
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "account")]
    pub account_id: String,
    /// Event timestamp
    pub time: DateTime<Utc>,
    /// Region of event
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    /// Region of event
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: String,
    /// Information about resources impacted by event
    pub resources: Vec<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub detail: HashMap<String, Value>,
}
