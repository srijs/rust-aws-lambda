/// `CodePipelineEvent` contains data from an event sent from AWS Codepipeline
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodePipelineEvent {
    #[serde(rename = "CodePipeline.job")]
    pub code_pipeline_job: CodePipelineJob,
}

/// `CodePipelineJob` represents a job from an AWS CodePipeline event
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodePipelineJob {
    pub id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    pub data: CodePipelineData,
}

/// `CodePipelineData` represents a job from an AWS CodePipeline event
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodePipelineData {
    #[serde(rename = "actionConfiguration")]
    pub action_configuration: CodePipelineActionConfiguration,
    #[serde(rename = "inputArtifacts")]
    pub input_artifacts: Vec<CodePipelineInputArtifact>,
    #[serde(rename = "outputArtifacts")]
    pub out_put_artifacts: Vec<CodePipelineOutputArtifact>,
    #[serde(rename = "artifactCredentials")]
    pub artifact_credentials: CodePipelineArtifactCredentials,
    #[serde(rename = "continuationToken")]
    pub continuation_token: String,
}

/// `CodePipelineActionConfiguration` represents an Action Configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodePipelineActionConfiguration {
    pub configuration: CodePipelineConfiguration,
}

/// `CodePipelineConfiguration` represents a configuration for an Action Configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodePipelineConfiguration {
    #[serde(rename = "FunctionName")]
    pub function_name: String,
    #[serde(rename = "UserParameters")]
    pub user_parameters: String,
}

/// `CodePipelineInputArtifact` represents an input artifact
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodePipelineInputArtifact {
    pub location: CodePipelineInputLocation,
    pub revision: Option<String>,
    pub name: String,
}

/// `CodePipelineInputLocation` represents a input location
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodePipelineInputLocation {
    #[serde(rename = "s3Location")]
    pub s3_location: CodePipelineS3Location,
    #[serde(rename = "type")]
    pub location_type: String,
}

/// `CodePipelineS3Location` represents an s3 input location
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodePipelineS3Location {
    #[serde(rename = "bucketName")]
    pub bucket_name: String,
    #[serde(rename = "objectKey")]
    pub object_key: String,
}

/// `CodePipelineOutputArtifact` represents an output artifact
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodePipelineOutputArtifact {
    pub location: CodePipelineInputLocation,
    pub revision: Option<String>,
    pub name: String,
}

/// `CodePipelineOutputLocation` represents a output location
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodePipelineOutputLocation {
    #[serde(rename = "s3Location")]
    pub s3_location: CodePipelineS3Location,
    #[serde(rename = "type")]
    pub location_type: String,
}

/// `CodePipelineArtifactCredentials` represents CodePipeline artifact credentials
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CodePipelineArtifactCredentials {
    #[serde(rename = "secretAccessKey")]
    pub secret_access_key: String,
    #[serde(rename = "sessionToken")]
    pub session_token: String,
    #[serde(rename = "accessKeyId")]
    pub access_key_id: String,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn deserializes_event() {
        let data = include_bytes!("fixtures/example-codepipeline_job-event.json");
        let _: CodePipelineEvent = serde_json::from_slice(data).unwrap();
    }
}
