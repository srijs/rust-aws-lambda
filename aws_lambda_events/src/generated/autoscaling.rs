use chrono::{DateTime, Utc};
use custom_serde::*;
use serde_json::Value;
use std::collections::HashMap;

/// `AutoScalingEvent` struct is used to parse the json for auto scaling event types //
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AutoScalingEvent {
    /// The version of event data
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    /// The unique ID of the event
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub id: Option<String>,
    /// Details about event type
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "detail-type")]
    pub detail_type: Option<String>,
    /// Source of the event
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub source: Option<String>,
    /// AccountId
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "account")]
    pub account_id: Option<String>,
    /// Event timestamp
    pub time: DateTime<Utc>,
    /// Region of event
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    /// Information about resources impacted by event
    pub resources: Vec<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    pub detail: HashMap<String, Value>,
}
