use custom_serde::*;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_json::Value;

/// `AppSyncResolverTemplate` represents the requests from AppSync to Lambda
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AppSyncResolverTemplate<T1=Value>
where T1: DeserializeOwned,
      T1: Serialize,
{
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    pub operation: AppSyncOperation,
    #[serde(bound="")]
    pub payload: T1,
}

pub type AppSyncOperation = String;
