use custom_serde::*;
use std::collections::HashMap;

/// `ConnectEvent` contains the data structure for a Connect event.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConnectEvent {
    #[serde(rename = "Details")]
    pub details: ConnectDetails,
    /// The name of the event.
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Name")]
    pub name: Option<String>,
}

/// `ConnectDetails` holds the details of a Connect event
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConnectDetails {
    #[serde(rename = "ContactData")]
    pub contact_data: ConnectContactData,
    /// The parameters that have been set in the Connect instance at the time of the Lambda invocation.
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "Parameters")]
    pub parameters: HashMap<String, String>,
}

/// `ConnectContactData` holds all of the contact information for the user that invoked the Connect event.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConnectContactData {
    /// The custom attributes from Connect that the Lambda function was invoked with.
    #[serde(deserialize_with = "deserialize_lambda_map")]
    #[serde(default)]
    #[serde(rename = "Attributes")]
    pub attributes: HashMap<String, String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Channel")]
    pub channel: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "ContactId")]
    pub contact_id: Option<String>,
    #[serde(rename = "CustomerEndpoint")]
    pub customer_endpoint: ConnectEndpoint,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "InitialContactId")]
    pub initial_contact_id: Option<String>,
    /// Either: INBOUND/OUTBOUND/TRANSFER/CALLBACK
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "InitiationMethod")]
    pub initiation_method: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "PreviousContactId")]
    pub previous_contact_id: Option<String>,
    #[serde(rename = "Queue")]
    pub queue: ConnectQueue,
    #[serde(rename = "SystemEndpoint")]
    pub system_endpoint: ConnectEndpoint,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "InstanceARN")]
    pub instance_arn: Option<String>,
}

/// `ConnectEndpoint` represents routing information.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConnectEndpoint {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Address")]
    pub address: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Type")]
    pub type_: Option<String>,
}

/// `ConnectQueue` represents a queue object.
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ConnectQueue {
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "ARN")]
    pub arn: Option<String>,
}

pub type ConnectResponse = HashMap<String, String>;

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_event() {
        let data = include_bytes!("fixtures/example-connect-event.json");
        let parsed: ConnectEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: ConnectEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
