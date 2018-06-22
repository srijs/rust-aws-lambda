use chrono::{DateTime, Utc};
use serde_json::Value;

/// `CloudWatchEvent` is the outer structure of an event sent via CloudWatch Events.
/// For examples of events that come via CloudWatch Events, see https://docs.aws.amazon.com/AmazonCloudWatch/latest/events/EventTypes.html
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CloudWatchEvent {
    pub version: String,
    pub id: String,
    #[serde(rename = "detail-type")]
    pub detail_type: String,
    pub source: String,
    #[serde(rename = "account")]
    pub account_id: String,
    pub time: DateTime<Utc>,
    pub region: String,
    pub resources: Vec<String>,
    pub detail: Value,
}
