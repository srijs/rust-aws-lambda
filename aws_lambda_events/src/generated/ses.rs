use chrono::{DateTime, Utc};

/// `SimpleEmailEvent` is the outer structure of an event sent via SES.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleEmailEvent {
    #[serde(rename = "Records")]
    pub records: Vec<SimpleEmailRecord>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleEmailRecord {
    #[serde(rename = "eventVersion")]
    pub event_version: String,
    #[serde(rename = "eventSource")]
    pub event_source: String,
    pub ses: SimpleEmailService,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleEmailService {
    pub mail: SimpleEmailMessage,
    pub receipt: SimpleEmailReceipt,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleEmailMessage {
    #[serde(rename = "commonHeaders")]
    pub common_headers: SimpleEmailCommonHeaders,
    pub source: String,
    pub timestamp: DateTime<Utc>,
    pub destination: Vec<String>,
    pub headers: Vec<SimpleEmailHeader>,
    #[serde(rename = "headersTruncated")]
    pub headers_truncated: bool,
    #[serde(rename = "messageId")]
    pub message_id: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleEmailReceipt {
    pub recipients: Vec<String>,
    pub timestamp: DateTime<Utc>,
    #[serde(rename = "spamVerdict")]
    pub spam_verdict: SimpleEmailVerdict,
    #[serde(rename = "dkimVerdict")]
    pub dkim_verdict: SimpleEmailVerdict,
    #[serde(rename = "spfVerdict")]
    pub spf_verdict: SimpleEmailVerdict,
    #[serde(rename = "virusVerdict")]
    pub virus_verdict: SimpleEmailVerdict,
    pub action: SimpleEmailReceiptAction,
    #[serde(rename = "processingTimeMillis")]
    pub processing_time_millis: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleEmailHeader {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleEmailCommonHeaders {
    pub from: Vec<String>,
    pub to: Vec<String>,
    #[serde(rename = "returnPath")]
    pub return_path: String,
    #[serde(rename = "messageId")]
    pub message_id: String,
    pub date: String,
    pub subject: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleEmailReceiptAction {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "invocationType")]
    pub invocation_type: String,
    #[serde(rename = "functionArn")]
    pub function_arn: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SimpleEmailVerdict {
    pub status: String,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn deserializes_event() {
        let data = include_bytes!("fixtures/example-ses-event.json");
        let _: SimpleEmailEvent = serde_json::from_slice(data).unwrap();
    }
}
