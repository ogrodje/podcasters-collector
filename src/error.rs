use std::error::Error;
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, PartialEq)]
pub enum NewAnchorError {
    HttpError(String),
    JSONParsingError(String),
    StringParsingError(String),
    TransformationFailed,
    NoCSRFToken,
}
