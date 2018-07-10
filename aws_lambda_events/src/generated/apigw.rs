use custom_serde::*;
use std::collections::HashMap;
use serde_json::Value;

/// `ApiGatewayProxyRequest` contains data coming from the API Gateway proxy
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayProxyRequest {
    /// The resource path defined in API Gateway
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub resource: Option<String>,
    /// The resource path defined in API Gateway
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub resource: String,
    /// The url path for the caller
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: Option<String>,
    /// The url path for the caller
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    pub headers: HashMap<String, String>,
    #[serde(rename = "queryStringParameters")]
    pub query_string_parameters: HashMap<String, String>,
    #[serde(rename = "pathParameters")]
    pub path_parameters: HashMap<String, String>,
    #[serde(rename = "stageVariables")]
    pub stage_variables: HashMap<String, String>,
    #[serde(rename = "requestContext")]
    pub request_context: ApiGatewayProxyRequestContext,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub body: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub body: String,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: Option<bool>,
}

/// `ApiGatewayProxyResponse` configures the response to be returned by API Gateway for the request
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayProxyResponse {
    #[serde(rename = "statusCode")]
    pub status_code: i64,
    pub headers: HashMap<String, String>,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub body: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub body: String,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: Option<bool>,
}

/// `ApiGatewayProxyRequestContext` contains the information to identify the AWS account and resources invoking the
/// Lambda function. It also includes Cognito identity information for the caller.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayProxyRequestContext {
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
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourceId")]
    pub resource_id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub stage: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub stage: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub identity: ApiGatewayRequestIdentity,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourcePath")]
    pub resource_path: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourcePath")]
    pub resource_path: String,
    pub authorizer: HashMap<String, Value>,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// The API Gateway rest API Id
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiId")]
    pub apiid: Option<String>,
    /// The API Gateway rest API Id
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiId")]
    pub apiid: String,
}

/// `ApiGatewayRequestIdentity` contains identity information for the request caller.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayRequestIdentity {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoIdentityPoolId")]
    pub cognito_identity_pool_id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoIdentityPoolId")]
    pub cognito_identity_pool_id: String,
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
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoIdentityId")]
    pub cognito_identity_id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoIdentityId")]
    pub cognito_identity_id: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub caller: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub caller: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiKey")]
    pub api_key: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "sourceIp")]
    pub source_ip: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "sourceIp")]
    pub source_ip: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoAuthenticationType")]
    pub cognito_authentication_type: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoAuthenticationType")]
    pub cognito_authentication_type: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoAuthenticationProvider")]
    pub cognito_authentication_provider: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "cognitoAuthenticationProvider")]
    pub cognito_authentication_provider: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userArn")]
    pub user_arn: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userArn")]
    pub user_arn: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userAgent")]
    pub user_agent: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub user: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub user: String,
}

/// `ApiGatewayCustomAuthorizerRequestTypeRequestIdentity` contains identity information for the request caller.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerRequestTypeRequestIdentity {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiKey")]
    pub api_key: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "sourceIp")]
    pub source_ip: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "sourceIp")]
    pub source_ip: String,
}

/// `ApiGatewayCustomAuthorizerContext` represents the expected format of an API Gateway custom authorizer response.
/// Deprecated. Code should be updated to use the Authorizer map from APIGatewayRequestIdentity. Ex: Authorizer["principalId"]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerContext {
    #[serde(rename = "principalId")]
    pub principal_id: Option<String>,
    #[serde(rename = "stringKey")]
    pub string_key: Option<String>,
    #[serde(rename = "numKey")]
    pub num_key: Option<i64>,
    #[serde(rename = "boolKey")]
    pub bool_key: Option<bool>,
}

/// `ApiGatewayCustomAuthorizerRequestTypeRequestContext` represents the expected format of an API Gateway custom authorizer response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerRequestTypeRequestContext {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: String,
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
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourceId")]
    pub resource_id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub stage: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub stage: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "requestId")]
    pub request_id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub identity: ApiGatewayCustomAuthorizerRequestTypeRequestIdentity,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourcePath")]
    pub resource_path: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resourcePath")]
    pub resource_path: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiId")]
    pub apiid: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "apiId")]
    pub apiid: String,
}

/// `ApiGatewayCustomAuthorizerRequest` contains data coming in to a custom API Gateway authorizer function.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerRequest {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "authorizationToken")]
    pub authorization_token: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "authorizationToken")]
    pub authorization_token: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "methodArn")]
    pub method_arn: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "methodArn")]
    pub method_arn: String,
}

/// `ApiGatewayCustomAuthorizerRequestTypeRequest` contains data coming in to a custom API Gateway authorizer function.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerRequestTypeRequest {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "methodArn")]
    pub method_arn: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "methodArn")]
    pub method_arn: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub resource: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub resource: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub path: String,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    pub headers: HashMap<String, String>,
    #[serde(rename = "queryStringParameters")]
    pub query_string_parameters: HashMap<String, String>,
    #[serde(rename = "pathParameters")]
    pub path_parameters: HashMap<String, String>,
    #[serde(rename = "stageVariables")]
    pub stage_variables: HashMap<String, String>,
    #[serde(rename = "requestContext")]
    pub request_context: ApiGatewayCustomAuthorizerRequestTypeRequestContext,
}

/// `ApiGatewayCustomAuthorizerResponse` represents the expected format of an API Gateway authorization response.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerResponse {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "principalId")]
    pub principal_id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "policyDocument")]
    pub policy_document: ApiGatewayCustomAuthorizerPolicy,
    pub context: Option<HashMap<String, Value>>,
    #[serde(rename = "usageIdentifierKey")]
    pub usage_identifier_key: Option<String>,
}

/// `ApiGatewayCustomAuthorizerPolicy` represents an IAM policy
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ApiGatewayCustomAuthorizerPolicy {
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: String,
    pub statement: Vec<IamPolicyStatement>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IamPolicyStatement {
    pub action: Vec<String>,
    #[cfg(feature = "string-null-none")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub effect: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub effect: String,
    pub resource: Vec<String>,
}
