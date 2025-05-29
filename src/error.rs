use serde::{Deserialize, Serialize};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(String),

    #[error("HTTP request error: {0}")]
    Http(String),

    #[error("JSON parsing error: {0}")]
    Json(String),

    #[error("PHP binary not found")]
    PhpBinaryNotFound,

    #[error("PHP server error: {0}")]
    PhpServer(String),

    #[error("Git error: {0}")]
    Git(String),

    #[error("Framework detection error: {0}")]
    FrameworkDetection(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Process error: {0}")]
    Process(String),

    #[error("Archive extraction error: {0}")]
    Archive(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err.to_string())
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err.to_string())
    }
}

impl From<git2::Error> for Error {
    fn from(err: git2::Error) -> Self {
        Error::Git(err.to_string())
    }
}

impl From<zip::result::ZipError> for Error {
    fn from(err: zip::result::ZipError) -> Self {
        Error::Archive(err.to_string())
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::InvalidUrl(err.to_string())
    }
}
