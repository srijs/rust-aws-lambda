use std::collections::HashMap;

/// `CognitoEvent` contains data from an event sent from AWS Cognito
#[derive(Debug, Clone, Deserialize, Serialize)]
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

/// `CognitoDatasetRecord` represents a record from an AWS Cognito event
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CognitoDatasetRecord {
    #[serde(rename = "newValue")]
    pub new_value: String,
    #[serde(rename = "oldValue")]
    pub old_value: String,
    pub op: String,
}
