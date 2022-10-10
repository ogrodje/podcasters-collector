use ureq::serde_json::{from_value as json_from_value, Value as JsonValue};
use ureq::Agent;
// use serde_derive::derive_deserialize as Deserialize;
use crate::config::{anchor_episodes_url, ANCHOR_CSRF_URL, ANCHOR_LOGIN_URL, ANCHOR_METADATA_URL};
use crate::Credentials;
use serde::Deserialize;
// use crate::credentials::Credentials;
use crate::error::{to_anchor_error, AnchorError};

// pub mod config;

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Metadata {
    pub webStationId: String,
}

#[allow(non_snake_case, dead_code)]
#[derive(Deserialize)]
pub struct Episode {
    // episodeId: u32,
    // webEpisodeId: String,
    shareLinkPath: String,
    pub title: String,
    pub totalPlays: u32,
}

type CSRFToken = String;

pub struct AnchorClient {
    pub(self) agent: Agent,
}

impl AnchorClient {
    pub fn from_agent(agent: Agent) -> Self {
        AnchorClient { agent }
    }

    fn parse_json(response: ureq::Response) -> Result<JsonValue, AnchorError> {
        response
            .into_json()
            .map_err(|e| AnchorError::JSONParsingError(e.to_string()))
    }

    fn parse_string(response: ureq::Response) -> Result<String, AnchorError> {
        response
            .into_string()
            .map_err(|e| AnchorError::StringParsingError(e.to_string()))
    }

    pub fn get_csrf_token(&mut self) -> Result<CSRFToken, AnchorError> {
        self.agent
            .get(ANCHOR_CSRF_URL)
            .call()
            .map_err(to_anchor_error)
            .and_then(AnchorClient::parse_json)
            .and_then(|json| {
                json["csrfToken"]
                    .as_str()
                    .map(String::from)
                    .ok_or(AnchorError::NoCSRFToken)
            })
    }

    pub fn post_login(
        &mut self,
        credentials: Credentials,
        token: CSRFToken,
    ) -> Result<(), AnchorError> {
        self.agent
            .post(ANCHOR_LOGIN_URL)
            .send_json(ureq::json!({
                "email": credentials.email,
                "password": credentials.password,
                "_csrf": token,
            }))
            .map_err(to_anchor_error)
            .and_then(AnchorClient::parse_string)
            .map(|_| ())
    }

    pub fn get_metadata(&mut self) -> Result<Metadata, AnchorError> {
        self.agent
            .get(ANCHOR_METADATA_URL)
            .call()
            .map_err(to_anchor_error)
            .and_then(AnchorClient::parse_json)
            .map(|v| json_from_value(v).unwrap())
    }

    fn get_episodes(&mut self, station_id: String) -> Result<JsonValue, AnchorError> {
        self.agent
            .get(&format!(anchor_episodes_url!(), station_id))
            .call()
            .map_err(to_anchor_error)
            .and_then(AnchorClient::parse_json)
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
