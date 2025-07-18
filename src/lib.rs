use std::collections::HashMap;
use url::Url;

use crate::exports::edgee::components::data_collection::{Dict, EdgeeRequest, Event, HttpMethod};
use exports::edgee::components::data_collection::{Data, Guest, PageData};
use posthog_payload::{PostHogEvent, Settings};
mod posthog_payload;

wit_bindgen::generate!({world: "data-collection", path: ".edgee/wit", generate_all});
export!(Component);

struct Component;

/*
* Implement the Guest trait for the Component struct
* to create the required functions for the data collection protocol
* for your provider.
* The functions are page, track, and user.
* The page function is called when the page event is triggered.
* The track function is called when the track event is triggered.
* The user function is called when the user event is triggered.
* The functions should return an EdgeeRequest or an error message.
* The EdgeeRequest contains the method, url, headers, and body of the request.
*/

impl Guest for Component {
    fn page(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        if let Data::Page(ref data) = edgee_event.data {
            let posthog_payload = Settings::new(settings_dict).map_err(|e| e.to_string())?;

            let mut event =
                PostHogEvent::new(&edgee_event, "$pageview").map_err(|e| e.to_string())?;
            event
                .properties
                .extend(extract_page_data(&edgee_event.context.page));
            event.properties.extend(extract_page_data(data));
            Ok(build_edgee_request(posthog_payload, event))
        } else {
            Err("Missing page data".to_string())
        }
    }

    #[allow(unused_variables)]
    fn track(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        if let Data::Track(ref data) = edgee_event.data {
            if data.name.is_empty() {
                return Err("Track name is not set".to_string());
            }
            let posthog_payload = Settings::new(settings_dict).map_err(|e| e.to_string())?;
            let track_data: HashMap<String, serde_json::Value> = data
                .properties
                .iter()
                .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                .collect();
            let mut event =
                PostHogEvent::new(&edgee_event, &data.name).map_err(|e| e.to_string())?;
            event.properties.extend(track_data);
            event
                .properties
                .extend(extract_page_data(&edgee_event.context.page));
            Ok(build_edgee_request(posthog_payload, event))
        } else {
            Err("Missing page data".to_string())
        }
    }

    #[allow(unused_variables)]
    fn user(edgee_event: Event, settings_dict: Dict) -> Result<EdgeeRequest, String> {
        if let Data::User(ref data) = edgee_event.data {
            let posthog_payload = Settings::new(settings_dict).map_err(|e| e.to_string())?;

            let mut event =
                PostHogEvent::new(&edgee_event, "$indentify").map_err(|e| e.to_string())?;

            // Create custom data
            let mut user_data: HashMap<String, serde_json::Value> = HashMap::new();

            user_data.insert(
                "$set".to_owned(),
                data.properties
                    .iter()
                    .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                    .collect(),
            );
            event
                .properties
                .extend(extract_page_data(&edgee_event.context.page));
            event.properties.extend(user_data);
            Ok(build_edgee_request(posthog_payload, event))
        } else {
            Err("Missing page data".to_string())
        }
    }
}

fn build_edgee_request(posthog_payload: Settings, event: PostHogEvent) -> EdgeeRequest {
    let headers = vec![(
        String::from("content-type"),
        String::from("application/json"),
    )];

    let posthog_map = serde_json::to_value(&event.posthog_data)
        .ok()
        .and_then(|v| v.as_object().cloned())
        .unwrap_or_default()
        .into_iter()
        .map(|(k, v)| (format!("${k}"), v));

    let merged_properties = posthog_map
        .chain(event.properties)
        .collect::<serde_json::Map<_, _>>();

    let body_payload = serde_json::json!({
        "properties": merged_properties,
        "event": event.event,
        "distinct_id": event.distinct_id,
        "api_key": posthog_payload.api_key,
    });

    let url = format!("https://{}.i.posthog.com/i/v0/e/", posthog_payload.region);
    EdgeeRequest {
        method: HttpMethod::Post,
        url,
        headers,
        forward_client_headers: true,
        body: { serde_json::to_string(&body_payload).unwrap() },
    }
}

fn extract_page_data(page_event: &PageData) -> HashMap<String, serde_json::Value> {
    let mut page_data: HashMap<String, serde_json::Value> = HashMap::new();

    let parsed_url = page_event.url.clone().parse::<Url>().unwrap();
    let parsed_referer: Option<Url> = if page_event.referrer.is_empty() {
        None
    } else {
        Url::parse(&page_event.referrer).ok()
    };

    page_data.insert(
        "$session_entry_url".to_owned(),
        serde_json::Value::String(parsed_url.to_string()),
    );

    page_data.insert(
        "$current_url".to_owned(),
        serde_json::Value::String(parsed_url.to_string()),
    );
    page_data.insert(
        "$session_entry_host".to_owned(),
        serde_json::Value::String(parsed_url.host_str().unwrap().to_string()),
    );
    page_data.insert(
        "$host".to_owned(),
        serde_json::Value::String(parsed_url.host_str().unwrap().to_string()),
    );
    page_data.insert(
        "$session_entry_pathname".to_owned(),
        serde_json::Value::String(page_event.path.clone()),
    );
    page_data.insert(
        "$pathname".to_owned(),
        serde_json::Value::String(page_event.path.clone()),
    );
    page_data.insert(
        "title".to_owned(),
        serde_json::Value::String(page_event.title.clone()),
    );
    if let Some(parsed_referer) = parsed_referer {
        page_data.insert(
            "$session_entry_referrer".to_owned(),
            serde_json::Value::String(parsed_referer.to_string()),
        );
        page_data.insert(
            "$session_entry_referring_domain".to_owned(),
            serde_json::Value::String(parsed_referer.domain().unwrap().to_string()),
        );
        page_data.insert(
            "$referring_domain".to_owned(),
            serde_json::Value::String(parsed_referer.domain().unwrap().to_string()),
        );
    }
    page_data
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::exports::edgee::components::data_collection::{
        Campaign, Client, Context, Data, EventType, PageData, Session, UserData,
    };
    use exports::edgee::components::data_collection::Consent;
    use pretty_assertions::assert_eq;
    use uuid::Uuid;

    fn sample_user_data(edgee_id: String) -> UserData {
        UserData {
            user_id: "123".to_string(),
            anonymous_id: "456".to_string(),
            edgee_id,
            properties: vec![
                ("prop1".to_string(), "value1".to_string()),
                ("prop2".to_string(), "10".to_string()),
            ],
        }
    }

    fn sample_context(edgee_id: String, locale: String, session_start: bool) -> Context {
        Context {
            page: sample_page_data(),
            user: sample_user_data(edgee_id),
            client: Client {
                city: "Paris".to_string(),
                ip: "192.168.0.1".to_string(),
                locale,
                timezone: "CET".to_string(),
                user_agent: "Chrome".to_string(),
                user_agent_architecture: "unknown".to_string(),
                user_agent_bitness: "64".to_string(),
                user_agent_full_version_list: "abc".to_string(),
                user_agent_version_list: "abc".to_string(),
                user_agent_mobile: "mobile".to_string(),
                user_agent_model: "don't know".to_string(),
                os_name: "MacOS".to_string(),
                os_version: "latest".to_string(),
                screen_width: 1024,
                screen_height: 768,
                screen_density: 2.0,
                continent: "Europe".to_string(),
                country_code: "FR".to_string(),
                country_name: "France".to_string(),
                region: "West Europe".to_string(),
            },
            campaign: Campaign {
                name: "random".to_string(),
                source: "random".to_string(),
                medium: "random".to_string(),
                term: "random".to_string(),
                content: "random".to_string(),
                creative_format: "random".to_string(),
                marketing_tactic: "random".to_string(),
            },
            session: Session {
                session_id: "random".to_string(),
                previous_session_id: "random".to_string(),
                session_count: 2,
                session_start,
                first_seen: 123,
                last_seen: 123,
            },
        }
    }

    fn sample_page_data() -> PageData {
        PageData {
            name: "page name".to_string(),
            category: "category".to_string(),
            keywords: vec!["value1".to_string(), "value2".into()],
            title: "page title".to_string(),
            url: "https://example.com/full-url?test=1".to_string(),
            path: "/full-path".to_string(),
            search: "?test=1".to_string(),
            referrer: "".to_string(),
            properties: vec![
                ("prop1".to_string(), "value1".to_string()),
                ("prop2".to_string(), "10".to_string()),
                ("currency".to_string(), "USD".to_string()),
            ],
        }
    }

    fn sample_page_event(
        consent: Option<Consent>,
        edgee_id: String,
        locale: String,
        session_start: bool,
    ) -> Event {
        Event {
            uuid: Uuid::new_v4().to_string(),
            timestamp: 123,
            timestamp_millis: 123,
            timestamp_micros: 123,
            event_type: EventType::Page,
            data: Data::Page(sample_page_data()),
            context: sample_context(edgee_id, locale, session_start),
            consent,
        }
    }

    #[test]
    fn page_works_fine() {
        let event = sample_page_event(
            Some(Consent::Granted),
            "abc".to_string(),
            "fr".to_string(),
            true,
        );
        let settings = vec![
            ("region".to_string(), "eu".to_string()),
            ("api_key".to_string(), "key".to_string()),
        ];
        let result = Component::page(event, settings);

        assert_eq!(result.is_err(), false);
        let edgee_request = result.unwrap();
        assert_eq!(edgee_request.method, HttpMethod::Post);
        assert_eq!(edgee_request.body.is_empty(), false);
        assert_eq!(
            edgee_request
                .url
                .starts_with("https://eu.i.posthog.com/i/v0/e/"),
            true
        );
        // add more checks (headers, querystring, etc.)
    }
}
