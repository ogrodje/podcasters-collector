use ureq::serde_json::Value as JsonValue;

use crate::error::AnchorError;

pub fn parse_json(response: ureq::Response) -> Result<JsonValue, AnchorError> {
    response
        .into_json()
        .map_err(|e| AnchorError::JSONParsingError(e.to_string()))
}

pub fn parse_string(response: ureq::Response) -> Result<String, AnchorError> {
    response
        .into_string()
        .map_err(|e| AnchorError::StringParsingError(e.to_string()))
}

pub fn to_anchor_error(error: ureq::Error) -> AnchorError {
    match error {
        ureq::Error::Transport(e) => AnchorError::HttpError(e.to_string()),
        other => AnchorError::HttpError(other.to_string()),
    }
}
