use crate::exports::edgee::components::data_collection::{Dict, Event};
use anyhow::Context;
use serde::Serialize;
use std::collections::HashMap;

pub struct Settings {
    pub region: String,
    pub api_key: String,
}

impl Settings {
    pub fn new(settings_dict: Dict) -> anyhow::Result<Self> {
        let settings_map: HashMap<String, String> = settings_dict
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect();

        let region = settings_map
            .get("region")
            .context("Missing Region setting")?
            .to_string();

        let api_key = settings_map
            .get("api_key")
            .context("Missing api_key setting")?
            .to_string();

        Ok(Self { region, api_key })
    }
}

#[derive(Serialize, Debug)]
pub struct PostHogEvent {
    pub event: String,
    pub distinct_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, serde_json::Value>>,
}

impl PostHogEvent {
    pub fn new(edgee_event: &Event, event_name: &str) -> anyhow::Result<Self> {
        let posthog_event = PostHogEvent {
            event: event_name.to_string(),
            distinct_id: edgee_event.context.user.user_id.clone(),
            properties: Some(HashMap::new()),
        };

        Ok(posthog_event)
    }
}
