/// `CloudwatchLogsEvent` represents raw data from a cloudwatch logs event
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CloudwatchLogsEvent {
    #[serde(rename = "awslogs")]
    pub aws_logs: CloudwatchLogsRawData,
}

/// `CloudwatchLogsRawData` contains gzipped base64 json representing the bulk
/// of a cloudwatch logs event
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CloudwatchLogsRawData {
    pub data: String,
}

/// `CloudwatchLogsData` is an unmarshal'd, ungzip'd, cloudwatch logs event
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CloudwatchLogsData {
    pub owner: String,
    #[serde(rename = "logGroup")]
    pub log_group: String,
    #[serde(rename = "logStream")]
    pub log_stream: String,
    #[serde(rename = "subscriptionFilters")]
    pub subscription_filters: Vec<String>,
    #[serde(rename = "messageType")]
    pub message_type: String,
    #[serde(rename = "logEvents")]
    pub log_events: Vec<CloudwatchLogsLogEvent>,
}

/// LogEvent represents a log entry from cloudwatch logs
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CloudwatchLogsLogEvent {
    pub id: String,
    pub timestamp: i64,
    pub message: String,
}
