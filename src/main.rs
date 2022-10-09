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

struct Anchor {
    agent: Agent,
}


impl Anchor {
    pub fn get_csrf_token(&mut self) -> Result<CSRFToken, AnchorError> {
        self.agent
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

    pub fn post_login(&mut self, credentials: Credentials, token: CSRFToken) -> Result<(), AnchorError> {
        self.agent
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

    pub fn get_metadata(&mut self) -> Result<Metadata, AnchorError> {
        self.agent
            .get(config::ANCHOR_METADATA_URL)
            .call()
            .map_err(to_anchor_error)
            .and_then(parse_json)
            .map(|v| json_from_value(v).unwrap())
    }

    fn get_episodes(&mut self, station_id: String) -> Result<JsonValue, AnchorError> {
        self.agent
            .get(&format!(anchor_episodes_url!(), station_id))
            .call()
            .map_err(to_anchor_error)
            .and_then(parse_json)
    }

    pub fn all_episodes(&mut self, station_id: String) -> Result<Vec<Episode>, AnchorError> {
        fn transform_episodes(items: &Vec<JsonValue>) -> Vec<Episode> {
            items
                .iter()
                .map(|i| json_from_value(i.clone()).unwrap())
                .collect()
        }

        self.get_episodes(station_id).and_then(|json| {
            json["items"]
                .as_array()
                .map(transform_episodes)
                .ok_or(AnchorError::TransformationFailed)
        })
    }
}

pub(crate) fn main() {
    let credentials = credentials::from_env();

    let agent: Agent = ureq::AgentBuilder::new()
        .timeout_read(Duration::from_secs(2))
        .timeout_write(Duration::from_secs(2))
        .build();

    let mut anchor = Anchor { agent };
    let _login = anchor.get_csrf_token()
        .and_then(|token| anchor.post_login(credentials, token));

    let episodes = anchor.get_metadata()
        .and_then(|metadata| anchor.all_episodes(metadata.webStationId));

    for Ok(episode) in episodes.iter() {
        println!("{} {}", episode.title, episode.totalPlays);
    }

    /*
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
     */
}
