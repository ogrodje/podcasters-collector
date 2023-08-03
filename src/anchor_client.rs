use serde::{Deserialize, Serialize};
use ureq::serde_json::{from_value as json_from_value, Value as JsonValue};
use ureq::{Agent, Response};

use crate::config::{anchor_episodes_url, ANCHOR_CSRF_URL, ANCHOR_LOGIN_URL, ANCHOR_METADATA_URL};
use crate::credentials::Credentials;
use crate::error::{to_anchor_error, AnchorError};

#[allow(non_snake_case)]
#[derive(Deserialize)]
pub struct Metadata {
    pub webStationId: String,
}

#[allow(non_snake_case, dead_code)]
#[derive(Serialize, Deserialize, Clone)]
pub struct Episode {
    // episodeId: u32,
    // webEpisodeId: String,
    shareLinkPath: String,
    pub title: String,
    pub totalPlays: u32,
}

type CSRFToken = String;
type ResponseResult<R> = Result<R, AnchorError>;

pub struct AnchorClient {
    pub(self) agent: Agent,
}

impl AnchorClient {
    pub fn from_agent(agent: Agent) -> Self {
        AnchorClient { agent }
    }

    fn parse<R, F, FE>(response: Response, handle: F, map_err: FE) -> ResponseResult<R>
    where
        F: FnOnce(Response) -> Result<R, std::io::Error>,
        FE: FnOnce(std::io::Error) -> AnchorError,
    {
        match response.status() {
            200..=201 => handle(response).map_err(map_err),
            code => Err(AnchorError::HttpError(format!(
                "Failed! Status code: {}",
                code
            ))),
        }
    }

    fn parse_json(response: Response) -> ResponseResult<JsonValue> {
        Self::parse(
            response,
            |r| r.into_json(),
            |e| AnchorError::JSONParsingError(e.to_string()),
        )
    }

    fn parse_string(response: Response) -> ResponseResult<String> {
        Self::parse(
            response,
            |r| r.into_string(),
            |e| AnchorError::StringParsingError(e.to_string()),
        )
    }

    pub fn get_csrf_token(&mut self) -> Result<CSRFToken, AnchorError> {
        fn read_token(json: &JsonValue) -> Result<CSRFToken, AnchorError> {
            json["csrfToken"]
                .as_str()
                .map(String::from)
                .ok_or(AnchorError::NoCSRFToken)
        }

        self.agent
            .get(ANCHOR_CSRF_URL)
            .call()
            .map_err(to_anchor_error)
            .and_then(AnchorClient::parse_json)
            .and_then(|json| read_token(&json))
    }

    pub fn post_login(
        &mut self,
        credentials: &Credentials,
        token: &CSRFToken,
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

    fn get_episodes(&mut self, station_id: &str) -> Result<JsonValue, AnchorError> {
        self.agent
            .get(&format!(anchor_episodes_url!(), station_id))
            .call()
            .map_err(to_anchor_error)
            .and_then(AnchorClient::parse_json)
    }

    pub fn all_episodes(&mut self, station_id: &str) -> Result<Vec<Episode>, AnchorError> {
        fn transform_episodes(items: &[JsonValue]) -> Vec<Episode> {
            items
                .iter()
                .map(|i| json_from_value(i.clone()).unwrap())
                .collect()
        }

        self.get_episodes(station_id).and_then(|json| {
            json["items"]
                .as_array()
                .map(|p| transform_episodes(p))
                .ok_or(AnchorError::TransformationFailed)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn parse_json_failure() {
        Response::new(200, "Ok", "broken-payload")
            .map_err(|e| AnchorError::HttpError(e.to_string()))
            .and_then(AnchorClient::parse_json)
            .expect("Boom");
    }

    #[test]
    #[should_panic]
    fn parse_json_failure_500() {
        Response::new(500, "Ok", "broken-payload")
            .map_err(|e| AnchorError::HttpError(e.to_string()))
            .and_then(AnchorClient::parse_json)
            .expect("Boom");
    }

    #[test]
    fn parse_json_ok() {
        let json_result = Response::new(200, "Ok", "{\"result\": \"ok\"}")
            .map_err(|e| AnchorError::HttpError(e.to_string()))
            .and_then(AnchorClient::parse_json)
            .unwrap();

        assert!(json_result["result"] == "ok", "Parsing JSON has failed.")
    }

    #[test]
    fn parse_string_ok() {
        let response = Response::new(200, "ok", "ok").unwrap();
        assert_eq!(AnchorClient::parse_string(response).unwrap(), "ok")
    }

    #[test]
    #[should_panic]
    fn parse_string_failure() {
        let response = Response::new(500, "ok", "x").unwrap();
        assert_eq!(AnchorClient::parse_string(response).unwrap(), "ok")
    }
}
