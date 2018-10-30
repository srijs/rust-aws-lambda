use custom_serde::*;
use chrono::{DateTime, Utc};

/// `SimpleEmailEvent` is the outer structure of an event sent via SES.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailEvent {
    #[serde(rename = "Records")]
    pub records: Vec<SimpleEmailRecord>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailRecord {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventVersion")]
    pub event_version: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "eventSource")]
    pub event_source: Option<String>,
    pub ses: SimpleEmailService,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailService {
    pub mail: SimpleEmailMessage,
    pub receipt: SimpleEmailReceipt,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailMessage {
    #[serde(rename = "commonHeaders")]
    pub common_headers: SimpleEmailCommonHeaders,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub source: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub destination: Vec<String>,
    pub headers: Vec<SimpleEmailHeader>,
    #[serde(rename = "headersTruncated")]
    pub headers_truncated: bool,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "messageId")]
    pub message_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailReceipt {
    pub recipients: Vec<String>,
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "spamVerdict")]
    pub spam_verdict: SimpleEmailVerdict,
    #[serde(rename = "dkimVerdict")]
    pub dkim_verdict: SimpleEmailVerdict,
    #[serde(rename = "dmarcVerdict")]
    pub dmarc_verdict: SimpleEmailVerdict,
    #[serde(rename = "dmarcPolicy")]
    pub dmarc_policy: SimpleEmailVerdict,
    #[serde(rename = "spfVerdict")]
    pub spf_verdict: SimpleEmailVerdict,
    #[serde(rename = "virusVerdict")]
    pub virus_verdict: SimpleEmailVerdict,
    pub action: SimpleEmailReceiptAction,
    #[serde(rename = "processingTimeMillis")]
    pub processing_time_millis: i64,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailHeader {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailCommonHeaders {
    pub from: Vec<String>,
    pub to: Vec<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "returnPath")]
    pub return_path: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "messageId")]
    pub message_id: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub date: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub subject: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailReceiptAction {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "invocationType")]
    pub invocation_type: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "functionArn")]
    pub function_arn: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailVerdict {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub status: Option<String>,
}

pub type SimpleEmailDispositionValue = String;

/// `SimpleEmailDisposition` disposition return for SES to control rule functions
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct SimpleEmailDisposition {
    pub disposition: SimpleEmailDispositionValue,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_event() {
        let data = include_bytes!("fixtures/example-ses-event.json");
        let parsed: SimpleEmailEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: SimpleEmailEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
