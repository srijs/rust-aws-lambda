use backtrace_parser::Backtrace;
use serde::{Serialize, Serializer};
use serde_bytes::Bytes;
use serde_schema::{types::Type, Schema, SchemaSerialize};

pub(crate) const SERVICE_METHOD_PING: &str = "Function.Ping";
pub(crate) const SERVICE_METHOD_INVOKE: &str = "Function.Invoke";

#[derive(Debug, Deserialize)]
pub(crate) struct PingRequest {}

#[derive(Debug, Serialize, SchemaSerialize)]
pub(crate) struct PingResponse {}

#[derive(Debug, Deserialize)]
#[serde(rename = "InvokeRequest_Timestamp")]
pub(crate) struct InvokeRequestTimestamp {
    #[serde(rename = "Seconds")]
    pub secs: i64,
    #[serde(rename = "Nanos")]
    pub nanos: i64,
}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Serialize, SchemaSerialize)]
#[cfg_attr(test, derive(Deserialize))]
pub(crate) enum InvokeResponse<'a> {
    #[serde(rename = "Payload", borrow)]
    Payload(Bytes<'a>),
    #[serde(rename = "Error", borrow)]
    Error(InvokeResponseError<'a>),
}

#[derive(Debug, Serialize, SchemaSerialize)]
#[cfg_attr(test, derive(Deserialize))]
#[serde(rename = "InvokeResponse_Error")]
pub(crate) struct InvokeResponseError<'a> {
    #[serde(rename = "Message")]
    pub message: &'a str,
    #[serde(rename = "Type")]
    pub type_: &'a str,
    #[serde(rename = "StackTrace", skip_serializing_if = "Option::is_none", skip_deserializing)]
    pub stack_trace: Option<InvokeResponseErrorStackTrace<'a>>,
    #[serde(rename = "ShouldExit", default)]
    pub should_exit: bool,
}

#[derive(Debug)]
pub(crate) struct InvokeResponseErrorStackTrace<'a>(pub Backtrace<'a>);

impl<'a> Serialize for InvokeResponseErrorStackTrace<'a> {
    fn serialize<S: Serializer>(&self, ser: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;

        let len = self.0.frames().map(|frame| frame.symbols().count()).sum();
        let mut seq = ser.serialize_seq(Some(len))?;

        for frame in self.0.frames() {
            for symbol in frame.symbols() {
                seq.serialize_element(&InvokeResponseErrorStackFrame {
                    path: symbol.filename().and_then(|p| p.to_str()),
                    line: symbol.lineno().map(|i| i as i32),
                    label: symbol.name().unwrap_or("<unknown>"),
                })?;
            }
        }
        seq.end()
    }
}

impl<'a> SchemaSerialize for InvokeResponseErrorStackTrace<'a> {
    fn schema_register<S: Schema>(schema: &mut S) -> Result<S::TypeId, S::Error> {
        let type_id = InvokeResponseErrorStackFrame::schema_register(schema)?;
        schema.register_type(Type::build().seq_type(None, type_id))
    }
}

#[derive(Clone, Debug, Serialize, SchemaSerialize)]
#[serde(rename = "InvokeResponse_Error_StackFrame")]
struct InvokeResponseErrorStackFrame<'a> {
    #[serde(rename = "Path")]
    path: Option<&'a str>,
    #[serde(rename = "Line")]
    line: Option<i32>,
    #[serde(rename = "Label")]
    label: &'a str,
}
