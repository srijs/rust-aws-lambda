use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct S3Event {
    #[serde(rename = "Records")]
    pub records: Vec<S3EventRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct S3EventRecord {
    #[serde(rename = "eventVersion")]
    pub event_version: String,
    #[serde(rename = "eventSource")]
    pub event_source: String,
    #[serde(rename = "awsRegion")]
    pub aws_region: String,
    #[serde(rename = "eventTime")]
    pub event_time: DateTime<Utc>,
    #[serde(rename = "eventName")]
    pub event_name: String,
    #[serde(rename = "userIdentity")]
    pub principal_id: S3UserIdentity,
    #[serde(rename = "requestParameters")]
    pub request_parameters: S3RequestParameters,
    #[serde(rename = "responseElements")]
    pub response_elements: HashMap<String, String>,
    pub s3: S3Entity,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct S3UserIdentity {
    #[serde(rename = "principalId")]
    pub principal_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct S3RequestParameters {
    #[serde(rename = "sourceIPAddress")]
    pub source_ip_address: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct S3Entity {
    #[serde(rename = "s3SchemaVersion")]
    pub schema_version: String,
    #[serde(rename = "configurationId")]
    pub configuration_id: String,
    pub bucket: S3Bucket,
    pub object: S3Object,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct S3Bucket {
    pub name: String,
    #[serde(rename = "ownerIdentity")]
    pub owner_identity: S3UserIdentity,
    pub arn: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct S3Object {
    pub key: String,
    pub size: i64,
    #[serde(rename = "urlDecodedKey")]
    pub url_decoded_key: String,
    #[serde(rename = "versionId")]
    pub version_id: String,
    #[serde(rename = "eTag")]
    pub e_tag: String,
    pub sequencer: String,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn deserializes_event() {
        let data = include_bytes!("fixtures/example-s3-event.json");
        let _: S3Event = serde_json::from_slice(data).unwrap();
    }
}
