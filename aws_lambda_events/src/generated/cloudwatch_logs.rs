use custom_serde::*;

/// `CloudwatchLogsEvent` represents raw data from a cloudwatch logs event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CloudwatchLogsEvent {
    #[serde(rename = "awslogs")]
    pub aws_logs: CloudwatchLogsRawData,
}

/// `CloudwatchLogsRawData` contains gzipped base64 json representing the bulk
/// of a cloudwatch logs event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CloudwatchLogsRawData {
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub data: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub data: String,
}

/// `CloudwatchLogsData` is an unmarshal'd, ungzip'd, cloudwatch logs event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CloudwatchLogsData {
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub owner: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub owner: String,
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "logGroup")]
    pub log_group: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "logGroup")]
    pub log_group: String,
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "logStream")]
    pub log_stream: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "logStream")]
    pub log_stream: String,
    #[serde(rename = "subscriptionFilters")]
    pub subscription_filters: Vec<String>,
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "messageType")]
    pub message_type: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "messageType")]
    pub message_type: String,
    #[serde(rename = "logEvents")]
    pub log_events: Vec<CloudwatchLogsLogEvent>,
}

/// LogEvent represents a log entry from cloudwatch logs
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CloudwatchLogsLogEvent {
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub id: String,
    pub timestamp: i64,
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub message: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub message: String,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_event() {
        let data = include_bytes!("fixtures/example-cloudwatch_logs-event.json");
        let parsed: CloudwatchLogsEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: CloudwatchLogsEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
