use chrono::{DateTime, Utc};
use custom_serde::*;

/// `CodeCommitEvent` represents a CodeCommit event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodeCommitEvent {
    #[serde(rename = "Records")]
    pub records: Vec<CodeCommitRecord>,
}

pub type CodeCommitEventTime = DateTime<Utc>;

/// represents a CodeCommit record
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodeCommitRecord {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventId")]
    pub event_id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventId")]
    pub event_id: String,
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
    #[serde(rename = "eventTime")]
    pub event_time: CodeCommitEventTime,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventTriggerName")]
    pub event_trigger_name: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventTriggerName")]
    pub event_trigger_name: String,
    #[serde(rename = "eventPartNumber")]
    pub event_part_number: u64,
    #[serde(rename = "codecommit")]
    pub code_commit: CodeCommitCodeCommit,
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
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventTriggerConfigId")]
    pub event_trigger_config_id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventTriggerConfigId")]
    pub event_trigger_config_id: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userIdentityARN")]
    pub user_identity_arn: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userIdentityARN")]
    pub user_identity_arn: String,
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
    #[serde(rename = "eventTotalParts")]
    pub event_total_parts: u64,
}

/// `CodeCommitCodeCommit` represents a CodeCommit object in a record
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodeCommitCodeCommit {
    pub references: Vec<CodeCommitReference>,
}

/// `CodeCommitReference` represents a Reference object in a CodeCommit object
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodeCommitReference {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub commit: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub commit: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "ref")]
    pub ref_: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "ref")]
    pub ref_: String,
    pub created: Option<bool>,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_event() {
        let data = include_bytes!("fixtures/example-code_commit-event.json");
        let parsed: CodeCommitEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: CodeCommitEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
