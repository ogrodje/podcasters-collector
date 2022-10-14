use std::error::Error;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, Clone, PartialEq)]
pub enum AnchorError {
    HttpError(String),
    JSONParsingError(String),
    StringParsingError(String),
    TransformationFailed,
    NoCSRFToken,
}

impl Display for AnchorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AnchorError::HttpError(str) => f.write_str(&format!("HTTP Error {}", str.to_string())),
            AnchorError::JSONParsingError(inner) => {
                f.write_str(&format!("JSON parsing has failed {}", inner.to_string()))
            }
            AnchorError::StringParsingError(inner) => {
                f.write_str(&format!("String parsing has failed {}", inner.to_string()))
            }
            other => f.write_str(&format!("Error with {}", other)),
        }
    }
}

pub fn to_anchor_error(error: ureq::Error) -> AnchorError {
    match error {
        ureq::Error::Transport(e) => AnchorError::HttpError(e.to_string()),
        other => AnchorError::HttpError(other.to_string()),
    }
}

impl From<ureq::Error> for AnchorError {
    fn from(error: ureq::Error) -> Self {
        to_anchor_error(error)
    }
}

/*
impl From<std::io::Error> for AnchorError {
    fn from(error: std::io::Error) -> Self {
        match error {
            e => AnchorError::StringParsingError(e.to_string()),
            e => AnchorError::JSONParsingError(e.to_string())
        }
    }
}
*/

impl Error for AnchorError {}
