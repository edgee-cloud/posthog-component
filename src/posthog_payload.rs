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
    pub properties: HashMap<String, serde_json::Value>,
    pub posthog_data: PostHogData,
}

impl PostHogEvent {
    pub fn new(edgee_event: &Event, event_name: &str) -> anyhow::Result<Self> {
        let mut posthog_event = PostHogEvent {
            event: event_name.to_string(),
            distinct_id: edgee_event.context.user.edgee_id.clone(),
            properties: HashMap::new(),
            posthog_data: PostHogData::default(),
        };

        let client = &edgee_event.context.client;

        if let Some((browser, version)) = first_browser_version(&client.user_agent_version_list) {
            posthog_event.posthog_data.browser = browser;
            posthog_event.posthog_data.browser_version = version.parse::<i64>().unwrap_or_default();
        }

        let locale = edgee_event.context.client.locale.clone();
        if locale.contains("-") {
            let parts: Vec<&str> = locale.split("-").collect();
            posthog_event.posthog_data.browser_language_prefix = parts[0].to_string();
            posthog_event.posthog_data.browser_language = parts[1].to_uppercase();
        } else {
            posthog_event.posthog_data.browser_language = locale.clone();
        }

        posthog_event.posthog_data.geoip_continent_name = Some(client.continent.clone());
        posthog_event.posthog_data.geoip_country_code = Some(client.country_code.clone());
        posthog_event.posthog_data.geoip_country_name = Some(client.country_name.clone());
        posthog_event.posthog_data.geoip_subdivision_1_name = Some(client.region.clone());
        posthog_event.posthog_data.geoip_city_name = Some(client.city.clone());
        posthog_event.posthog_data.geoip_time_zone = Some(client.timezone.clone());

        posthog_event.posthog_data.ip = Some(client.ip.clone());
        posthog_event.posthog_data.os = Some(client.os_name.clone());
        posthog_event.posthog_data.os_version = Some(client.os_version.clone());
        if edgee_event.context.client.screen_width.is_positive() {
            posthog_event.posthog_data.screen_width =
                edgee_event.context.client.screen_width as i64;
            posthog_event.posthog_data.viewport_width =
                edgee_event.context.client.screen_width as i64;
        }
        if edgee_event.context.client.screen_height.is_positive() {
            posthog_event.posthog_data.screen_height =
                edgee_event.context.client.screen_height as i64;
            posthog_event.posthog_data.viewport_height =
                edgee_event.context.client.screen_height as i64;
        }

        posthog_event.posthog_data.raw_user_agent = Some(client.user_agent.clone());

        Ok(posthog_event)
    }
}

fn first_browser_version(input: &str) -> Option<(String, String)> {
    input
        .split('|')
        .next()
        .and_then(|part| part.split_once(';'))
        .map(|(browser, version)| (browser.to_string(), version.to_string()))
}

#[derive(Serialize, Debug, Default)]
pub(crate) struct PostHogData {
    pub browser: String,
    pub browser_language: String,
    pub browser_language_prefix: String,
    pub browser_version: i64,

    pub screen_height: i64,
    pub screen_width: i64,
    pub viewport_height: i64,
    pub viewport_width: i64,

    pub session_id: Option<String>,
    pub ip: Option<String>,
    pub os: Option<String>,
    pub os_version: Option<String>,

    pub geoip_continent_name: Option<String>,
    pub geoip_country_code: Option<String>,
    pub geoip_country_name: Option<String>,
    pub geoip_city_name: Option<String>,
    pub geoip_subdivision_1_name: Option<String>,
    pub geoip_postal_code: Option<String>,
    pub geoip_time_zone: Option<String>,

    pub raw_user_agent: Option<String>,
}
