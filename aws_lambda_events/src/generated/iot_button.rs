use custom_serde::*;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct IoTButtonEvent {
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "serialNumber")]
    pub serial_number: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "serialNumber")]
    pub serial_number: String,
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "clickType")]
    pub click_type: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "clickType")]
    pub click_type: String,
    #[cfg(not(feature = "string-null-empty"))]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "batteryVoltage")]
    pub battery_voltage: Option<String>,
    #[cfg(feature = "string-null-empty")]
    #[serde(deserialize_with = "deserialize_lambda_string")]
    #[serde(default)]
    #[serde(rename = "batteryVoltage")]
    pub battery_voltage: String,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn example_event() {
        let data = include_bytes!("fixtures/example-iot_button-event.json");
        let parsed: IoTButtonEvent = serde_json::from_slice(data).unwrap();
        let output: String = serde_json::to_string(&parsed).unwrap();
        let reparsed: IoTButtonEvent = serde_json::from_slice(output.as_bytes()).unwrap();
        assert_eq!(parsed, reparsed);
    }
}
