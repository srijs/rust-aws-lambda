use chrono::{DateTime, Utc};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SnsEvent {
    #[serde(rename = "Records")]
    pub records: Vec<SnsEventRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SnsEventRecord {
    #[serde(rename = "EventVersion")]
    pub event_version: String,
    #[serde(rename = "EventSubscriptionArn")]
    pub event_subscription_arn: String,
    #[serde(rename = "EventSource")]
    pub event_source: String,
    #[serde(rename = "Sns")]
    pub sns: SnsEntity,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SnsEntity {
    #[serde(rename = "Signature")]
    pub signature: String,
    #[serde(rename = "MessageId")]
    pub message_id: String,
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "TopicArn")]
    pub topic_arn: String,
    #[serde(rename = "MessageAttributes")]
    pub message_attributes: HashMap<String, Value>,
    #[serde(rename = "SignatureVersion")]
    pub signature_version: String,
    #[serde(rename = "Timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "SigningCertUrl")]
    pub signing_cert_url: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "UnsubscribeUrl")]
    pub unsubscribe_url: String,
    #[serde(rename = "Subject")]
    pub subject: String,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_event() {
        let data = include_bytes!("fixtures/example-sns-event.json");
        let parsed: SnsEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: SnsEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
