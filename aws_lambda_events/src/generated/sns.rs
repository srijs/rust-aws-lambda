use custom_serde::*;
use chrono::{DateTime, Utc};
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SnsEvent {
    #[serde(rename = "Records")]
    pub records: Vec<SnsEventRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SnsEventRecord {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "EventVersion")]
    pub event_version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "EventSubscriptionArn")]
    pub event_subscription_arn: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "EventSource")]
    pub event_source: Option<String>,
    #[serde(rename = "Sns")]
    pub sns: SnsEntity,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SnsEntity<T1=Value>
where T1: DeserializeOwned,
      T1: Serialize,
{
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Signature")]
    pub signature: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "MessageId")]
    pub message_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Type")]
    pub type_: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "TopicArn")]
    pub topic_arn: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(bound="")]
    #[serde(rename = "MessageAttributes")]
    pub message_attributes: HashMap<String, T1>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "SignatureVersion")]
    pub signature_version: Option<String>,
    #[serde(rename = "Timestamp")]
    pub timestamp: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "SigningCertUrl")]
    pub signing_cert_url: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Message")]
    pub message: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "UnsubscribeUrl")]
    pub unsubscribe_url: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Subject")]
    pub subject: Option<String>,
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
