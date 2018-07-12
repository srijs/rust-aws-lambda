use custom_serde::*;

/// `ConfigEvent` contains data from an event sent from AWS Config
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConfigEvent {
    /// The ID of the AWS account that owns the rule
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    /// The ID of the AWS account that owns the rule
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// The ARN that AWS Config assigned to the rule
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "configRuleArn")]
    pub config_rule_arn: Option<String>,
    /// The ARN that AWS Config assigned to the rule
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "configRuleArn")]
    pub config_rule_arn: String,
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "configRuleId")]
    pub config_rule_id: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "configRuleId")]
    pub config_rule_id: String,
    /// The name that you assigned to the rule that caused AWS Config to publish the event
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "configRuleName")]
    pub config_rule_name: Option<String>,
    /// The name that you assigned to the rule that caused AWS Config to publish the event
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "configRuleName")]
    pub config_rule_name: String,
    /// A boolean value that indicates whether the AWS resource to be evaluated has been removed from the rule's scope
    #[serde(rename = "eventLeftScope")]
    pub event_left_scope: bool,
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "executionRoleArn")]
    pub execution_role_arn: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "executionRoleArn")]
    pub execution_role_arn: String,
    /// If the event is published in response to a resource configuration change, this value contains a JSON configuration item
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "invokingEvent")]
    pub invoking_event: Option<String>,
    /// If the event is published in response to a resource configuration change, this value contains a JSON configuration item
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "invokingEvent")]
    pub invoking_event: String,
    /// A token that the function must pass to AWS Config with the PutEvaluations call
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resultToken")]
    pub result_token: Option<String>,
    /// A token that the function must pass to AWS Config with the PutEvaluations call
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "resultToken")]
    pub result_token: String,
    /// Key/value pairs that the function processes as part of its evaluation logic
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "ruleParameters")]
    pub rule_parameters: Option<String>,
    /// Key/value pairs that the function processes as part of its evaluation logic
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "ruleParameters")]
    pub rule_parameters: String,
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    pub version: String,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_event() {
        let data = include_bytes!("fixtures/example-config-event.json");
        let parsed: ConfigEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: ConfigEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
