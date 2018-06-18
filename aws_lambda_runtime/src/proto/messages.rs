use serde_bytes::Bytes;

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct PingRequest {}

#[derive(Clone, Debug, Serialize, SchemaSerialize)]
pub(crate) struct PingResponse {}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename = "InvokeRequest_Timestamp")]
pub(crate) struct InvokeRequestTimestamp {
    #[serde(rename = "Seconds")]
    pub secs: i64,
    #[serde(rename = "Nanos")]
    pub nanos: i64,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct InvokeRequest<'a> {
    #[serde(rename = "Payload", borrow)]
    pub payload: Bytes<'a>,
    #[serde(rename = "RequestId")]
    pub request_id: String,
    #[serde(rename = "XAmznTraceId", borrow)]
    pub x_amzn_trace_id: &'a str,
    #[serde(rename = "Deadline")]
    pub deadline: InvokeRequestTimestamp,
    #[serde(rename = "InvokedFunctionArn")]
    pub invoked_function_arn: String,
    #[serde(rename = "CognitoIdentityId", default)]
    pub cognito_identity_id: Option<String>,
    #[serde(rename = "CognitoIdentityPoolId", default)]
    pub cognito_identity_pool_id: Option<String>,
    #[serde(rename = "ClientContext", borrow, default)]
    pub client_context: Option<Bytes<'a>>,
}

#[derive(Clone, Debug, Serialize, SchemaSerialize)]
pub(crate) struct InvokeResponse<'a> {
    #[serde(rename = "Payload")]
    pub payload: Bytes<'a>,
    #[serde(rename = "Error", skip_serializing_if = "Option::is_none")]
    pub error: Option<InvokeResponseError>,
}

#[derive(Clone, Debug, Serialize, SchemaSerialize)]
#[serde(rename = "InvokeResponse_Error")]
pub(crate) struct InvokeResponseError {
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "StackTrace", skip_serializing_if = "Option::is_none")]
    pub stack_trace: Option<Vec<InvokeResponseErrorStackFrame>>,
    #[serde(rename = "ShouldExit")]
    pub should_exit: bool,
}

#[derive(Clone, Debug, Serialize, SchemaSerialize)]
#[serde(rename = "InvokeResponse_Error_StackFrame")]
pub(crate) struct InvokeResponseErrorStackFrame {
    #[serde(rename = "Path")]
    pub path: String,
    #[serde(rename = "Line")]
    pub line: i32,
    #[serde(rename = "Label")]
    pub label: String,
}
