use std::collections::HashMap;
use super::super::encodings::Base64Data;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SqsEvent {
    #[serde(rename = "Records")]
    pub records: Vec<SqsMessage>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SqsMessage {
    #[serde(rename = "messageId")]
    pub message_id: String,
    #[serde(rename = "receiptHandle")]
    pub receipt_handle: String,
    pub body: String,
    #[serde(rename = "md5OfBody")]
    pub md5_of_body: String,
    #[serde(rename = "md5OfMessageAttributes")]
    pub md5_of_message_attributes: String,
    pub attributes: HashMap<String, String>,
    #[serde(rename = "messageAttributes")]
    pub message_attributes: HashMap<String, SqsMessageAttribute>,
    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: String,
    #[serde(rename = "eventSource")]
    pub event_source: String,
    #[serde(rename = "awsRegion")]
    pub aws_region: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SqsMessageAttribute {
    #[serde(rename = "stringValue")]
    pub string_value: Option<String>,
    #[serde(rename = "binaryValue")]
    pub binary_value: Option<Base64Data>,
    #[serde(rename = "stringListValues")]
    pub string_list_values: Vec<String>,
    #[serde(rename = "binaryListValues")]
    pub binary_list_values: Vec<Base64Data>,
    #[serde(rename = "dataType")]
    pub data_type: String,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_event() {
        let data = include_bytes!("fixtures/example-sqs-event.json");
        let parsed: SqsEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: SqsEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
