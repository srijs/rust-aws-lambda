use std::collections::HashMap;

/// Metadata about the calling application.
#[derive(Deserialize)]
pub struct ClientApplication {
    pub(crate) installation_id: String,
    pub(crate) app_title: String,
    pub(crate) app_version_code: String,
    pub(crate) app_package_name: String,
}

/// Information about the client application passed by the calling application.
#[derive(Deserialize)]
pub struct ClientContext {
    #[serde(rename = "Client")]
    pub(crate) client: ClientApplication,
    pub(crate) env: HashMap<String, String>,
    pub(crate) custom: HashMap<String, String>,
}

/// The cognito identity used by the calling application.
#[derive(Deserialize)]
pub struct CognitoIdentity {
    #[serde(rename = "CognitoIdentityID", default)]
    pub(crate) cognito_identity_id: Option<String>,
    #[serde(rename = "CognitoIdentityPoolID", default)]
    pub(crate) cognito_identity_pool_id: Option<String>,
}

/// The set of metadata that is passed for every Invoke.
#[derive(Deserialize)]
pub struct LambdaContext {
    #[serde(rename = "AwsRequestID")]
    pub(crate) aws_request_id: String,
    #[serde(rename = "InvokedFunctionArn")]
    pub(crate) invoked_function_arn: String,
    #[serde(rename = "Identity")]
    pub(crate) identity: CognitoIdentity,
    #[serde(rename = "ClientContext")]
    pub(crate) client_context: Option<ClientContext>,
}
