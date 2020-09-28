use std::error::Error;
use std::fmt;
use std::io;
use toml::{de, ser};

#[derive(Debug)]
pub enum IssuersError {
    Request(reqwest::Error),
    TomlSerialization(ser::Error),
    TomlDeserialization(de::Error),
    Fs(io::Error),
}

impl Error for IssuersError {
    fn description(&self) -> &str {
        match *self {
            IssuersError::Request(..) => "request failed",
            IssuersError::TomlSerialization(..) => "toml serialization failed",
            IssuersError::TomlDeserialization(..) => "toml deserialization failed",
            IssuersError::Fs(..) => "filesystem operation failed",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            IssuersError::Request(ref e) => Some(e),
            IssuersError::TomlSerialization(ref e) => Some(e),
            IssuersError::TomlDeserialization(ref e) => Some(e),
            IssuersError::Fs(ref e) => Some(e),
        }
    }
}

impl fmt::Display for IssuersError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            IssuersError::Request(ref e) => write!(f, "{}", e),
            IssuersError::TomlSerialization(ref e) => write!(f, "{}", e),
            IssuersError::TomlDeserialization(ref e) => write!(f, "{}", e),
            IssuersError::Fs(ref e) => write!(f, "{}", e),
        }
    }
}

impl From<reqwest::Error> for IssuersError {
    fn from(e: reqwest::Error) -> Self {
        IssuersError::Request(e)
    }
}

impl From<ser::Error> for IssuersError {
    fn from(e: ser::Error) -> Self {
        IssuersError::TomlSerialization(e)
    }
}

impl From<de::Error> for IssuersError {
    fn from(e: de::Error) -> Self {
        IssuersError::TomlDeserialization(e)
    }
}

impl From<io::Error> for IssuersError {
    fn from(e: io::Error) -> Self {
        IssuersError::Fs(e)
    }
}
