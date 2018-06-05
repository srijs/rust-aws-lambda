/// `ConfigEvent` contains data from an event sent from AWS Config
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigEvent {
    /// The ID of the AWS account that owns the rule
    #[serde(rename = "accountId")]
    pub account_id: String,
    /// The ARN that AWS Config assigned to the rule
    #[serde(rename = "configRuleArn")]
    pub config_rule_arn: String,
    #[serde(rename = "configRuleId")]
    pub config_rule_id: String,
    /// The name that you assigned to the rule that caused AWS Config to publish the event
    #[serde(rename = "configRuleName")]
    pub config_rule_name: String,
    /// A boolean value that indicates whether the AWS resource to be evaluated has been removed from the rule's scope
    #[serde(rename = "eventLeftScope")]
    pub event_left_scope: bool,
    #[serde(rename = "executionRoleArn")]
    pub execution_role_arn: String,
    /// If the event is published in response to a resource configuration change, this value contains a JSON configuration item
    #[serde(rename = "invokingEvent")]
    pub invoking_event: String,
    /// A token that the function must pass to AWS Config with the PutEvaluations call
    #[serde(rename = "resultToken")]
    pub result_token: String,
    /// Key/value pairs that the function processes as part of its evaluation logic
    #[serde(rename = "ruleParameters")]
    pub rule_parameters: String,
    pub version: String,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn deserializes_event() {
        let data = include_bytes!("fixtures/example-config-event.json");
        let _: ConfigEvent = serde_json::from_slice(data).unwrap();
    }
}
