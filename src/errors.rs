use std::error::Error;
use std::fmt;
use std::io;
use toml::{de, ser};

#[derive(Debug)]
pub enum IssuesError {
    Request(reqwest::Error),
}

impl Error for IssuesError {
    fn description(&self) -> &str {
        match *self {
            IssuesError::Request(..) => "request failed",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            IssuesError::Request(ref e) => Some(e),
        }
    }
}

impl fmt::Display for IssuesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            IssuesError::Request(ref e) => write!(f, "{}", e),
        }
    }
}

impl From<reqwest::Error> for IssuesError {
    fn from(e: reqwest::Error) -> Self {
        IssuesError::Request(e)
    }
}

#[derive(Debug)]
pub enum HistoryError {
    TomlSerialization(ser::Error),
    TomlDeserialization(de::Error),
    Fs(io::Error),
}

impl Error for HistoryError {
    fn description(&self) -> &str {
        match *self {
            HistoryError::TomlSerialization(..) => "toml serialization failed",
            HistoryError::TomlDeserialization(..) => "toml deserialization failed",
            HistoryError::Fs(..) => "filesystem operation failed",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            HistoryError::TomlSerialization(ref e) => Some(e),
            HistoryError::TomlDeserialization(ref e) => Some(e),
            HistoryError::Fs(ref e) => Some(e),
        }
    }
}

impl fmt::Display for HistoryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            HistoryError::TomlSerialization(ref e) => write!(f, "{}", e),
            HistoryError::TomlDeserialization(ref e) => write!(f, "{}", e),
            HistoryError::Fs(ref e) => write!(f, "{}", e),
        }
    }
}

impl From<ser::Error> for HistoryError {
    fn from(e: ser::Error) -> Self {
        HistoryError::TomlSerialization(e)
    }
}

impl From<de::Error> for HistoryError {
    fn from(e: de::Error) -> Self {
        HistoryError::TomlDeserialization(e)
    }
}

impl From<io::Error> for HistoryError {
    fn from(e: io::Error) -> Self {
        HistoryError::Fs(e)
    }
}
