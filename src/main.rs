use std::borrow::Borrow;
use std::time::Duration;

use serde::Deserialize;
use ureq::serde_json::from_value as json_from_value;
use ureq::serde_json::Value as JsonValue;
use ureq::Agent;

use crate::anchor_client::{parse_json, parse_string, to_anchor_error};
use crate::config::anchor_episodes_url;
use crate::credentials::Credentials;
use crate::error::AnchorError;

pub mod anchor_client;
pub mod config;
pub mod credentials;
pub mod error;

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Metadata {
    webStationId: String,
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize)]
struct Episode {
    // episodeId: u32,
    // webEpisodeId: String,
    shareLinkPath: String,
    title: String,
    totalPlays: u32,
}

type CSRFToken = String;

fn get_csrf_token(agent: &Agent) -> Result<CSRFToken, AnchorError> {
    agent
        .get(config::ANCHOR_CSRF_URL)
        .call()
        .map_err(to_anchor_error)
        .and_then(parse_json)
        .and_then(|json| {
            json["csrfToken"]
                .as_str()
                .map(String::from)
                .ok_or(AnchorError::NoCSRFToken)
        })
}

fn post_login(
    agent: &Agent,
    credentials: Credentials,
    token: CSRFToken,
) -> Result<(), AnchorError> {
    agent
        .post(config::ANCHOR_LOGIN_URL)
        .send_json(ureq::json!({
            "email": credentials.email,
            "password": credentials.password,
            "_csrf": token,
        }))
        .map_err(to_anchor_error)
        .and_then(parse_string)
        .map(|_| ())
}

fn get_metadata(agent: &Agent) -> Result<Metadata, AnchorError> {
    agent
        .get(config::ANCHOR_METADATA_URL)
        .call()
        .map_err(to_anchor_error)
        .and_then(parse_json)
        .map(|v| json_from_value(v).unwrap())
}

fn get_episodes(agent: &Agent, station_id: String) -> Result<JsonValue, AnchorError> {
    agent
        .get(&format!(anchor_episodes_url!(), station_id))
        .call()
        .map_err(to_anchor_error)
        .and_then(parse_json)
}

fn all_episodes(agent: &Agent, station_id: String) -> Result<Vec<Episode>, AnchorError> {
    fn transform_episodes(items: &Vec<JsonValue>) -> Vec<Episode> {
        items
            .iter()
            .map(|i| json_from_value(i.clone()).unwrap())
            .collect()
    }

    get_episodes(agent, station_id).and_then(|json| {
        json["items"]
            .as_array()
            .map(transform_episodes)
            .ok_or(AnchorError::TransformationFailed)
    })
}

pub(crate) fn main() {
    let credentials = credentials::from_env();

    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(5))
        .timeout_write(Duration::from_secs(10))
        .build();

    let _token =
        get_csrf_token(agent.borrow()).and_then(|t| post_login(agent.borrow(), credentials, t));

    let metadata_result = get_metadata(agent.borrow());

    let episodes_result =
        metadata_result.and_then(|m| all_episodes(agent.borrow(), m.webStationId));

    let _x = episodes_result.map(|episodes| {
        for episode in episodes.iter() {
            println!("{} {}", episode.title, episode.totalPlays);
        }
    });
}
