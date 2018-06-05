use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;

/// `AutoScalingEvent` struct is used to parse the json for auto scaling event types //
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AutoScalingEvent {
    /// The version of event data
    pub version: String,
    /// The unique ID of the event
    pub id: String,
    /// Details about event type
    #[serde(rename = "detail-type")]
    pub detail_type: String,
    /// Source of the event
    pub source: String,
    /// AccountId
    #[serde(rename = "account")]
    pub account_id: String,
    /// Event timestamp
    pub time: DateTime<Utc>,
    /// Region of event
    pub region: String,
    /// Information about resources impacted by event
    pub resources: Vec<String>,
    pub detail: HashMap<String, Value>,
}
