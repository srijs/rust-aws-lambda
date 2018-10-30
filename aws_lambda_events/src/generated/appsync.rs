use custom_serde::*;
use serde_json::Value;

/// `AppSyncResolverTemplate` represents the requests from AppSync to Lambda
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AppSyncResolverTemplate {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    pub operation: AppSyncOperation,
    pub payload: Value,
}

pub type AppSyncOperation = String;
