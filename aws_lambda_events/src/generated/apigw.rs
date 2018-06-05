use std::collections::HashMap;
use serde_json::Value;

/// `APIGatewayProxyRequest` contains data coming from the API Gateway proxy
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIGatewayProxyRequest {
    /// The resource path defined in API Gateway
    pub resource: String,
    /// The url path for the caller
    pub path: String,
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
    pub request_context: APIGatewayProxyRequestContext,
    pub body: String,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: Option<bool>,
}

/// `APIGatewayProxyResponse` configures the response to be returned by API Gateway for the request
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIGatewayProxyResponse {
    #[serde(rename = "statusCode")]
    pub status_code: i64,
    pub headers: HashMap<String, String>,
    pub body: String,
    #[serde(rename = "isBase64Encoded")]
    pub is_base64_encoded: Option<bool>,
}

/// `APIGatewayProxyRequestContext` contains the information to identify the AWS account and resources invoking the
/// Lambda function. It also includes Cognito identity information for the caller.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIGatewayProxyRequestContext {
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    pub stage: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub identity: APIGatewayRequestIdentity,
    #[serde(rename = "resourcePath")]
    pub resource_path: String,
    pub authorizer: HashMap<String, Value>,
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    /// The API Gateway rest API Id
    #[serde(rename = "apiId")]
    pub apiid: String,
}

/// `APIGatewayRequestIdentity` contains identity information for the request caller.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIGatewayRequestIdentity {
    #[serde(rename = "cognitoIdentityPoolId")]
    pub cognito_identity_pool_id: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "cognitoIdentityId")]
    pub cognito_identity_id: String,
    pub caller: String,
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "sourceIp")]
    pub source_ip: String,
    #[serde(rename = "cognitoAuthenticationType")]
    pub cognito_authentication_type: String,
    #[serde(rename = "cognitoAuthenticationProvider")]
    pub cognito_authentication_provider: String,
    #[serde(rename = "userArn")]
    pub user_arn: String,
    #[serde(rename = "userAgent")]
    pub user_agent: String,
    pub user: String,
}

/// `APIGatewayCustomAuthorizerRequestTypeRequestIdentity` contains identity information for the request caller.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIGatewayCustomAuthorizerRequestTypeRequestIdentity {
    #[serde(rename = "apiKey")]
    pub api_key: String,
    #[serde(rename = "sourceIp")]
    pub source_ip: String,
}

/// `APIGatewayCustomAuthorizerContext` represents the expected format of an API Gateway custom authorizer response.
/// Deprecated. Code should be updated to use the Authorizer map from APIGatewayRequestIdentity. Ex: Authorizer["principalId"]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIGatewayCustomAuthorizerContext {
    #[serde(rename = "principalId")]
    pub principal_id: Option<String>,
    #[serde(rename = "stringKey")]
    pub string_key: Option<String>,
    #[serde(rename = "numKey")]
    pub num_key: Option<i64>,
    #[serde(rename = "boolKey")]
    pub bool_key: Option<bool>,
}

/// `APIGatewayCustomAuthorizerRequestTypeRequestContext` represents the expected format of an API Gateway custom authorizer response.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIGatewayCustomAuthorizerRequestTypeRequestContext {
    pub path: String,
    #[serde(rename = "accountId")]
    pub account_id: String,
    #[serde(rename = "resourceId")]
    pub resource_id: String,
    pub stage: String,
    #[serde(rename = "requestId")]
    pub request_id: String,
    pub identity: APIGatewayCustomAuthorizerRequestTypeRequestIdentity,
    #[serde(rename = "resourcePath")]
    pub resource_path: String,
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    #[serde(rename = "apiId")]
    pub apiid: String,
}

/// `APIGatewayCustomAuthorizerRequest` contains data coming in to a custom API Gateway authorizer function.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIGatewayCustomAuthorizerRequest {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "authorizationToken")]
    pub authorization_token: String,
    #[serde(rename = "methodArn")]
    pub method_arn: String,
}

/// `APIGatewayCustomAuthorizerRequestTypeRequest` contains data coming in to a custom API Gateway authorizer function.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIGatewayCustomAuthorizerRequestTypeRequest {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "methodArn")]
    pub method_arn: String,
    pub resource: String,
    pub path: String,
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
    pub request_context: APIGatewayCustomAuthorizerRequestTypeRequestContext,
}

/// `APIGatewayCustomAuthorizerResponse` represents the expected format of an API Gateway authorization response.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIGatewayCustomAuthorizerResponse {
    #[serde(rename = "principalId")]
    pub principal_id: String,
    #[serde(rename = "policyDocument")]
    pub policy_document: APIGatewayCustomAuthorizerPolicy,
    pub context: Option<HashMap<String, Value>>,
    #[serde(rename = "usageIdentifierKey")]
    pub usage_identifier_key: Option<String>,
}

/// `APIGatewayCustomAuthorizerPolicy` represents an IAM policy
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct APIGatewayCustomAuthorizerPolicy {
    pub version: String,
    pub statement: Vec<IAMPolicyStatement>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct IAMPolicyStatement {
    pub action: Vec<String>,
    pub effect: String,
    pub resource: Vec<String>,
}
