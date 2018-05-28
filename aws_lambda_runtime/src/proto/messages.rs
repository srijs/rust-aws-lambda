use serde_bytes::{ByteBuf, Bytes};

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
    #[serde(rename = "RequestId", borrow)]
    pub request_id: &'a str,
    #[serde(rename = "XAmznTraceId", borrow)]
    pub x_amzn_trace_id: &'a str,
    #[serde(rename = "Deadline")]
    pub deadline: InvokeRequestTimestamp,
    #[serde(rename = "InvokedFunctionArn", borrow)]
    pub invoked_function_arn: &'a str,
    #[serde(rename = "CognitoIdentityId", borrow, default)]
    pub cognito_identity_id: Option<&'a str>,
    #[serde(rename = "CognitoIdentityPoolId", borrow, default)]
    pub cognito_identity_pool_id: Option<&'a str>,
    #[serde(rename = "ClientContext", borrow, default)]
    pub client_context: Option<Bytes<'a>>,
}

#[derive(Clone, Debug, Serialize, SchemaSerialize)]
pub(crate) struct InvokeResponse {
    #[serde(rename = "Payload")]
    pub payload: ByteBuf,
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
