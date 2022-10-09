use std::borrow::Borrow;
use std::env;
use std::time::Duration;

use serde::Deserialize;
use ureq::serde_json::Value as JsonValue;
use ureq::Agent;
use ureq::{serde_json};

use crate::error::NewAnchorError;
pub mod error;

#[derive(Debug)]
struct Credentials {
    email: String,
    password: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct Metadata {
    webStationId: String,
}

#[allow(non_snake_case)]
#[derive(Deserialize)]
struct Episode {
    // episodeId: u32,
    // webEpisodeId: String,
    shareLinkPath: String,
    title: String,
    totalPlays: u32,
}

fn credentials_from_env() -> Credentials {
    Credentials {
        email: env::var("ANCHOR_EMAIL").expect("$ANCHOR_EMAIL is not set!"),
        password: env::var("ANCHOR_PASSWORD").expect("$ANCHOR_PASSWORD is not set!"),
    }
}

type CSRFToken = String;


// impl Error for NewAnchorError {}

fn parse_json(response: ureq::Response) -> Result<JsonValue, NewAnchorError> {
    response
        .into_json()
        .map_err(|e| NewAnchorError::JSONParsingError(e.to_string()))
}

fn parse_string(response: ureq::Response) -> Result<String, NewAnchorError> {
    response
        .into_string()
        .map_err(|e| NewAnchorError::StringParsingError(e.to_string()))
}

fn to_anchor_error(error: ureq::Error) -> NewAnchorError {
    match error {
        ureq::Error::Transport(e) => NewAnchorError::HttpError(e.to_string()),
        other => NewAnchorError::HttpError(other.to_string()),
    }
}

fn get_csrf_token(agent: &Agent) -> Result<CSRFToken, NewAnchorError> {
    agent
        .get("https://anchor.fm/api/csrf")
        .call()
        .map_err(to_anchor_error)
        .and_then(parse_json)
        .and_then(|json: JsonValue| {
            json["csrfToken"]
                .as_str()
                .map(String::from)
                .ok_or(NewAnchorError::NoCSRFToken)
        })
}

fn post_login(
    agent: &Agent,
    credentials: Credentials,
    token: CSRFToken,
) -> Result<(), NewAnchorError> {
    agent
        .post("https://anchor.fm/api/login")
        .send_json(ureq::json!({
            "email": credentials.email,
            "password": credentials.password,
            "_csrf": token,
        }))
        .map_err(to_anchor_error)
        .and_then(parse_string)
        .map(|_| ())
}

fn get_metadata(agent: &Agent) -> Result<Metadata, NewAnchorError> {
    agent
        .get("https://anchor.fm/api/podcast/metadata")
        .call()
        .map_err(to_anchor_error)
        .and_then(parse_json)
        .map(|v| serde_json::from_value(v).unwrap())
}

fn get_episodes(agent: &Agent, station_id: String) -> Result<JsonValue, NewAnchorError> {
    agent
        .get(
            format!(
                "https://anchor.fm/api/proxy/v3/stations/\
        webStationId:{}/episodePage?limit=50&orderBy=publishOn",
                station_id
            )
                .as_str(),
        )
        .call()
        .map_err(to_anchor_error)
        .and_then(parse_json)
}

fn all_episodes(agent: &Agent, station_id: String) -> Result<Vec<Episode>, NewAnchorError> {
    fn transform_episodes(items: &Vec<JsonValue>) -> Vec<Episode> {
        items
            .iter()
            .map(|i| serde_json::from_value(i.clone()).unwrap())
            .collect()
    }

    get_episodes(agent, station_id).and_then(|json| {
        json["items"]
            .as_array()
            .map(transform_episodes)
            .ok_or(NewAnchorError::TransformationFailed)
    })
}

pub(crate) fn main() {
    let credentials = credentials_from_env();

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
            println!("{},{}", episode.title, episode.totalPlays);
        }
    });
}
