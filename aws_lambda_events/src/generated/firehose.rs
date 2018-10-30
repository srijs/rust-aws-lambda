use custom_serde::*;
use super::super::encodings::{Base64Data, MillisecondTimestamp};

/// `KinesisFirehoseEvent` represents the input event from Amazon Kinesis Firehose. It is used as the input parameter.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisFirehoseEvent {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "invocationId")]
    pub invocation_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "deliveryStreamArn")]
    pub delivery_stream_arn: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    pub records: Vec<KinesisFirehoseEventRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisFirehoseEventRecord {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "recordId")]
    pub record_id: Option<String>,
    #[serde(rename = "approximateArrivalTimestamp")]
    pub approximate_arrival_timestamp: MillisecondTimestamp,
    pub data: Base64Data,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisFirehoseResponse {
    pub records: Vec<KinesisFirehoseResponseRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisFirehoseResponseRecord {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "recordId")]
    pub record_id: Option<String>,
    /// The status of the transformation. May be TransformedStateOk, TransformedStateDropped or TransformedStateProcessingFailed
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub result: Option<String>,
    pub data: Base64Data,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_event() {
        let data = include_bytes!("fixtures/example-firehose-event.json");
        let parsed: KinesisFirehoseEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: KinesisFirehoseEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
