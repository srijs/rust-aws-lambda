use std::collections::HashMap;

/// `CognitoEvent` contains data from an event sent from AWS Cognito Sync
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEvent {
    #[serde(rename = "datasetName")]
    pub dataset_name: String,
    #[serde(rename = "datasetRecords")]
    pub dataset_records: HashMap<String, CognitoDatasetRecord>,
    #[serde(rename = "eventType")]
    pub event_type: String,
    #[serde(rename = "identityId")]
    pub identity_id: String,
    #[serde(rename = "identityPoolId")]
    pub identity_pool_id: String,
    pub region: String,
    pub version: i64,
}

/// `CognitoDatasetRecord` represents a record from an AWS Cognito Sync event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoDatasetRecord {
    #[serde(rename = "newValue")]
    pub new_value: String,
    #[serde(rename = "oldValue")]
    pub old_value: String,
    pub op: String,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_event() {
        let data = include_bytes!("fixtures/example-cognito-event.json");
        let parsed: CognitoEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: CognitoEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
