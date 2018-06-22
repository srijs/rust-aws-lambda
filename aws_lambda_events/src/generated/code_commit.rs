use chrono::{DateTime, Utc};

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
    #[serde(rename = "eventId")]
    pub event_id: String,
    #[serde(rename = "eventVersion")]
    pub event_version: String,
    #[serde(rename = "eventTime")]
    pub event_time: CodeCommitEventTime,
    #[serde(rename = "eventTriggerName")]
    pub event_trigger_name: String,
    #[serde(rename = "eventPartNumber")]
    pub event_part_number: u64,
    #[serde(rename = "codecommit")]
    pub code_commit: CodeCommitCodeCommit,
    #[serde(rename = "eventName")]
    pub event_name: String,
    #[serde(rename = "eventTriggerConfigId")]
    pub event_trigger_config_id: String,
    #[serde(rename = "eventSourceARN")]
    pub event_source_arn: String,
    #[serde(rename = "userIdentityARN")]
    pub user_identity_arn: String,
    #[serde(rename = "eventSource")]
    pub event_source: String,
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
    pub commit: String,
    #[serde(rename = "ref")]
    pub ref_: String,
    pub created: Option<bool>,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn deserializes_event() {
        let data = include_bytes!("fixtures/example-code_commit-event.json");
        let _: CodeCommitEvent = serde_json::from_slice(data).unwrap();
    }
}
