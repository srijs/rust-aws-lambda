use chrono::{DateTime, Utc};
use custom_serde::*;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct S3Event {
    #[serde(rename = "Records")]
    pub records: Vec<S3EventRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct S3EventRecord {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventVersion")]
    pub event_version: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventVersion")]
    pub event_version: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventSource")]
    pub event_source: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventSource")]
    pub event_source: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "awsRegion")]
    pub aws_region: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "awsRegion")]
    pub aws_region: String,
    #[serde(rename = "eventTime")]
    pub event_time: DateTime<Utc>,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventName")]
    pub event_name: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
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
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "principalId")]
    pub principal_id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "principalId")]
    pub principal_id: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct S3RequestParameters {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "sourceIPAddress")]
    pub source_ip_address: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "sourceIPAddress")]
    pub source_ip_address: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct S3Entity {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "s3SchemaVersion")]
    pub schema_version: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "s3SchemaVersion")]
    pub schema_version: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "configurationId")]
    pub configuration_id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "configurationId")]
    pub configuration_id: String,
    pub bucket: S3Bucket,
    pub object: S3Object,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct S3Bucket {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub name: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub name: String,
    #[serde(rename = "ownerIdentity")]
    pub owner_identity: S3UserIdentity,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub arn: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub arn: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct S3Object {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub key: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub key: String,
    pub size: i64,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "urlDecodedKey")]
    pub url_decoded_key: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "urlDecodedKey")]
    pub url_decoded_key: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "versionId")]
    pub version_id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "versionId")]
    pub version_id: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eTag")]
    pub e_tag: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eTag")]
    pub e_tag: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub sequencer: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub sequencer: String,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_event() {
        let data = include_bytes!("fixtures/example-s3-event.json");
        let parsed: S3Event = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: S3Event = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
