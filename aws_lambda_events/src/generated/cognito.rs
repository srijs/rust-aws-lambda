use custom_serde::*;
use std::collections::HashMap;

/// `CognitoEvent` contains data from an event sent from AWS Cognito Sync
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEvent {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "datasetName")]
    pub dataset_name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "datasetRecords")]
    pub dataset_records: HashMap<String, CognitoDatasetRecord>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventType")]
    pub event_type: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "identityId")]
    pub identity_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "identityPoolId")]
    pub identity_pool_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    pub version: i64,
}

/// `CognitoDatasetRecord` represents a record from an AWS Cognito Sync event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoDatasetRecord {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "newValue")]
    pub new_value: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "oldValue")]
    pub old_value: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub op: Option<String>,
}

/// `CognitoEventUserPoolsPreSignup` is sent by AWS Cognito User Pools when a user attempts to register
/// (sign up), allowing a Lambda to perform custom validation to accept or deny the registration request
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPreSignup {
    #[serde(flatten)]
    pub cognito_event_user_pools_header: CognitoEventUserPoolsHeader,
    pub request: CognitoEventUserPoolsPreSignupRequest,
    pub response: CognitoEventUserPoolsPreSignupResponse,
}

/// `CognitoEventUserPoolsPostConfirmation` is sent by AWS Cognito User Pools after a user is confirmed,
/// allowing the Lambda to send custom messages or add custom logic.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPostConfirmation {
    #[serde(flatten)]
    pub cognito_event_user_pools_header: CognitoEventUserPoolsHeader,
    pub request: CognitoEventUserPoolsPostConfirmationRequest,
    pub response: CognitoEventUserPoolsPostConfirmationResponse,
}

/// `CognitoEventUserPoolsCallerContext` contains information about the caller
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsCallerContext {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "awsSdkVersion")]
    pub awssdk_version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "clientId")]
    pub client_id: Option<String>,
}

/// `CognitoEventUserPoolsHeader` contains common data from events sent by AWS Cognito User Pools
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsHeader {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "triggerSource")]
    pub trigger_source: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub region: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userPoolId")]
    pub user_pool_id: Option<String>,
    #[serde(rename = "callerContext")]
    pub caller_context: CognitoEventUserPoolsCallerContext,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userName")]
    pub user_name: Option<String>,
}

/// `CognitoEventUserPoolsPreSignupRequest` contains the request portion of a PreSignup event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPreSignupRequest {
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "userAttributes")]
    pub user_attributes: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "validationData")]
    pub validation_data: HashMap<String, String>,
}

/// `CognitoEventUserPoolsPreSignupResponse` contains the response portion of a PreSignup event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPreSignupResponse {
    #[serde(rename = "autoConfirmUser")]
    pub auto_confirm_user: bool,
    #[serde(rename = "autoVerifyEmail")]
    pub auto_verify_email: bool,
    #[serde(rename = "autoVerifyPhone")]
    pub auto_verify_phone: bool,
}

/// `CognitoEventUserPoolsPostConfirmationRequest` contains the request portion of a PostConfirmation event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPostConfirmationRequest {
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "userAttributes")]
    pub user_attributes: HashMap<String, String>,
}

/// `CognitoEventUserPoolsPostConfirmationResponse` contains the response portion of a PostConfirmation event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CognitoEventUserPoolsPostConfirmationResponse;

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
