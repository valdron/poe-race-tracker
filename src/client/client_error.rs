use super::race_event::EventParseError;
use std::num::ParseIntError;
use std::io;
use serde_json;

pub type ClientResult<T> = Result<T, ClientError>;

#[derive(Debug, Fail)]
pub enum ClientError {
    #[fail(display = "IoError: {}", err)]
    IoError { err: io::Error },
    #[fail(display = "StringError: {}", message)]
    StringError { message: String },
    #[fail(display = "ParseIntError: {:?}", err)]
    ParseError { err: ParseIntError },
    #[fail(display = "EventParseError")]
    EventParseError,
}

impl From<String> for ClientError {
    fn from(message: String) -> Self {
        ClientError::StringError { message }
    }
}

impl From<io::Error> for ClientError {
    fn from(err: io::Error) -> Self {
        ClientError::IoError { err }
    }
}

impl From<ParseIntError> for ClientError {
    fn from(err: ParseIntError) -> Self {
        ClientError::ParseError { err }
    }
}

impl From<EventParseError> for ClientError {
    fn from(_: EventParseError) -> Self {
        ClientError::EventParseError
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(err: serde_json::Error) -> Self {
        ClientError::StringError { message: format!("{:?}", err) }
    }
}
