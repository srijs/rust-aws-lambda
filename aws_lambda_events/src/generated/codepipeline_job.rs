use custom_serde::*;

/// `CodePipelineEvent` contains data from an event sent from AWS Codepipeline
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodePipelineEvent {
    #[serde(rename = "CodePipeline.job")]
    pub code_pipeline_job: CodePipelineJob,
}

/// `CodePipelineJob` represents a job from an AWS CodePipeline event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodePipelineJob {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub id: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub data: CodePipelineData,
}

/// `CodePipelineData` represents a job from an AWS CodePipeline event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodePipelineData {
    #[serde(rename = "actionConfiguration")]
    pub action_configuration: CodePipelineActionConfiguration,
    #[serde(rename = "inputArtifacts")]
    pub input_artifacts: Vec<CodePipelineInputArtifact>,
    #[serde(rename = "outputArtifacts")]
    pub out_put_artifacts: Vec<CodePipelineOutputArtifact>,
    #[serde(rename = "artifactCredentials")]
    pub artifact_credentials: CodePipelineArtifactCredentials,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "continuationToken")]
    pub continuation_token: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "continuationToken")]
    pub continuation_token: String,
}

/// `CodePipelineActionConfiguration` represents an Action Configuration
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodePipelineActionConfiguration {
    pub configuration: CodePipelineConfiguration,
}

/// `CodePipelineConfiguration` represents a configuration for an Action Configuration
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodePipelineConfiguration {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "FunctionName")]
    pub function_name: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "UserParameters")]
    pub user_parameters: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "UserParameters")]
    pub user_parameters: String,
}

/// `CodePipelineInputArtifact` represents an input artifact
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodePipelineInputArtifact {
    pub location: CodePipelineInputLocation,
    pub revision: Option<String>,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub name: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub name: String,
}

/// `CodePipelineInputLocation` represents a input location
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodePipelineInputLocation {
    #[serde(rename = "s3Location")]
    pub s3_location: CodePipelineS3Location,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub location_type: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub location_type: String,
}

/// `CodePipelineS3Location` represents an s3 input location
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodePipelineS3Location {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "bucketName")]
    pub bucket_name: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "objectKey")]
    pub object_key: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "objectKey")]
    pub object_key: String,
}

/// `CodePipelineOutputArtifact` represents an output artifact
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodePipelineOutputArtifact {
    pub location: CodePipelineInputLocation,
    pub revision: Option<String>,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub name: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub name: String,
}

/// `CodePipelineOutputLocation` represents a output location
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodePipelineOutputLocation {
    #[serde(rename = "s3Location")]
    pub s3_location: CodePipelineS3Location,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub location_type: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub location_type: String,
}

/// `CodePipelineArtifactCredentials` represents CodePipeline artifact credentials
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CodePipelineArtifactCredentials {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "sessionToken")]
    pub session_token: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "sessionToken")]
    pub session_token: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accessKeyId")]
    pub access_key_id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_event() {
        let data = include_bytes!("fixtures/example-codepipeline_job-event.json");
        let parsed: CodePipelineEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: CodePipelineEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
