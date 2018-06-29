use super::super::custom_serde::{Base64Data, SecondTimestamp};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisEvent {
    #[serde(rename = "Records")]
    pub records: Vec<KinesisEventRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisEventRecord {
    #[serde(rename = "awsRegion")]
    pub aws_region: String,
    #[serde(rename = "eventID")]
    pub event_id: String,
    #[serde(rename = "eventName")]
    pub event_name: String,
    #[serde(rename = "eventSource")]
    pub event_source: String,
    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: String,
    #[serde(rename = "eventVersion")]
    pub event_version: String,
    #[serde(rename = "invokeIdentityArn")]
    pub invoke_identity_arn: String,
    pub kinesis: KinesisRecord,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisRecord {
    #[serde(rename = "approximateArrivalTimestamp")]
    pub approximate_arrival_timestamp: SecondTimestamp,
    pub data: Base64Data,
    #[serde(rename = "encryptionType")]
    pub encryption_type: Option<String>,
    #[serde(rename = "partitionKey")]
    pub partition_key: String,
    #[serde(rename = "sequenceNumber")]
    pub sequence_number: String,
    #[serde(rename = "kinesisSchemaVersion")]
    pub kinesis_schema_version: String,
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
