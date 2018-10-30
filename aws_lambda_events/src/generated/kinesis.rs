use custom_serde::*;
use super::super::encodings::{Base64Data, SecondTimestamp};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisEvent {
    #[serde(rename = "Records")]
    pub records: Vec<KinesisEventRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisEventRecord {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "awsRegion")]
    pub aws_region: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventID")]
    pub event_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventName")]
    pub event_name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventSource")]
    pub event_source: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventVersion")]
    pub event_version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "invokeIdentityArn")]
    pub invoke_identity_arn: Option<String>,
    pub kinesis: KinesisRecord,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisRecord {
    #[serde(rename = "approximateArrivalTimestamp")]
    pub approximate_arrival_timestamp: SecondTimestamp,
    pub data: Base64Data,
    #[serde(rename = "encryptionType")]
    pub encryption_type: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "partitionKey")]
    pub partition_key: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "kinesisSchemaVersion")]
    pub kinesis_schema_version: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_event() {
        let data = include_bytes!("fixtures/example-kinesis-event.json");
        let parsed: KinesisEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: KinesisEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
