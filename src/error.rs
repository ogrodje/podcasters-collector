use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub enum AnchorError {
    HttpError(String),
    JSONParsingError(String),
    StringParsingError(String),
    TransformationFailed,
    NoCSRFToken,
}

impl fmt::Display for AnchorError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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

impl Error for AnchorError {}
