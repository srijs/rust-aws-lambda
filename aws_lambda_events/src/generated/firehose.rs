use chrono::{DateTime, Utc};
use super::super::custom_serde::*;

/// `KinesisFirehoseEvent` represents the input event from Amazon Kinesis Firehose. It is used as the input parameter.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisFirehoseEvent {
    #[serde(rename = "invocationId")]
    pub invocation_id: String,
    #[serde(rename = "deliveryStreamArn")]
    pub delivery_stream_arn: String,
    pub region: String,
    pub records: Vec<KinesisFirehoseEventRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct KinesisFirehoseEventRecord {
    #[serde(rename = "recordId")]
    pub record_id: String,
    #[serde(deserialize_with = "deserialize_milliseconds")]
    #[serde(serialize_with = "serialize_milliseconds")]
    #[serde(rename = "approximateArrivalTimestamp")]
    pub approximate_arrival_timestamp: DateTime<Utc>,
    #[serde(deserialize_with = "deserialize_base64")]
    #[serde(serialize_with = "serialize_base64")]
    pub data: Vec<u8>,
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
