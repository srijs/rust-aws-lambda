use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LexEvent {
    #[serde(rename = "messageVersion")]
    pub message_version: Option<String>,
    #[serde(rename = "invocationSource")]
    pub invocation_source: Option<String>,
    #[serde(rename = "userId")]
    pub user_id: Option<String>,
    #[serde(rename = "inputTranscript")]
    pub input_transcript: Option<String>,
    #[serde(rename = "sessionAttributes")]
    pub session_attributes: HashMap<String, String>,
    #[serde(rename = "requestAttributes")]
    pub request_attributes: Option<HashMap<String, String>>,
    pub bot: Option<LexBot>,
    #[serde(rename = "outputDialogMode")]
    pub output_dialog_mode: Option<String>,
    #[serde(rename = "currentIntent")]
    pub current_intent: Option<LexCurrentIntent>,
    #[serde(rename = "dialogAction")]
    pub dialog_action: Option<LexDialogAction>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LexBot {
    pub name: String,
    pub alias: String,
    pub version: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LexCurrentIntent {
    pub name: String,
    pub slots: Slots,
    #[serde(rename = "slotDetails")]
    pub slot_details: HashMap<String, SlotDetail>,
    #[serde(rename = "confirmationStatus")]
    pub confirmation_status: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SlotDetail {
    pub resolutions: Vec<HashMap<String, String>>,
    #[serde(rename = "originalValue")]
    pub original_value: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LexDialogAction {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "fulfillmentState")]
    pub fulfillment_state: String,
    pub message: HashMap<String, String>,
    #[serde(rename = "intentName")]
    pub intent_name: String,
    pub slots: Slots,
    #[serde(rename = "slotToElicit")]
    pub slot_to_elicit: String,
    #[serde(rename = "responseCard")]
    pub response_card: LexResponseCard,
}

pub type Slots = HashMap<String, String>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LexResponseCard {
    pub version: i64,
    #[serde(rename = "contentType")]
    pub content_type: String,
    #[serde(rename = "genericAttachments")]
    pub generic_attachments: Vec<Attachment>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Attachment {
    pub title: String,
    #[serde(rename = "subTitle")]
    pub sub_title: String,
    #[serde(rename = "imageUrl")]
    pub image_url: String,
    #[serde(rename = "attachmentLinkUrl")]
    pub attachment_link_url: String,
    pub buttons: Vec<HashMap<String, String>>,
}

#[cfg(test)]
mod test {
    use super::*;

    extern crate serde_json;

    #[test]
    fn deserializes_event() {
        let data = include_bytes!("fixtures/example-lex-event.json");
        let _: LexEvent = serde_json::from_slice(data).unwrap();
    }
}
