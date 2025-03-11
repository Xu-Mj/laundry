use argon2::password_hash;
use printpdf::image_crate;
use serde::{Deserialize, Serialize};
use std::{error::Error as StdError, fmt};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ErrorKind {
    UnknownError,
    DbError,
    ConfigReadError,
    ConfigParseError,
    NotFound,
    ParseError,
    InternalServer,
    IOError,
    ReqwestError,
    BadRequest,
    PrintError,
    PrinterNotSet,
    PrinterNotFound,
    InvalidPassword,
    AccountOrPassword,
    AccountNotRegister,
    UnAuthorized,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    kind: ErrorKind,
    details: Option<String>,
    #[serde(skip)]
    source: Option<Box<dyn StdError + Send + Sync>>,
}

impl Error {
    #[inline]
    pub fn new(
        kind: ErrorKind,
        details: impl Into<String>,
        source: impl StdError + 'static + Send + Sync,
    ) -> Self {
        Self {
            kind,
            source: Some(Box::new(source)),
            details: Some(details.into()),
        }
    }

    pub fn kind(&self) -> ErrorKind {
        self.kind.clone()
    }

    #[inline]
    pub fn with_kind(kind: ErrorKind) -> Self {
        Self {
            kind,
            source: None,
            details: None,
        }
    }

    #[inline]
    pub fn raw(error: Box<dyn StdError + 'static + Send + Sync>) -> Self {
        Self {
            kind: ErrorKind::InternalServer,
            details: Some(error.to_string()),
            source: Some(error),
        }
    }

    #[inline]
    pub fn with_details(kind: ErrorKind, details: impl Into<String>) -> Self {
        Self {
            kind,
            source: None,
            details: Some(details.into()),
        }
    }

    #[inline]
    pub fn internal(details: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::InternalServer,
            source: None,
            details: Some(details.into()),
        }
    }

    #[inline]
    pub fn not_found(details: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::NotFound,
            source: None,
            details: Some(details.into()),
        }
    }

    #[inline]
    pub fn bad_request(details: impl Into<String>) -> Self {
        Self {
            kind: ErrorKind::BadRequest,
            source: None,
            details: Some(details.into()),
        }
    }
    
    #[inline]
    pub fn account_or_pwd() -> Self {
        Self::with_kind(ErrorKind::AccountOrPassword)
    }

}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.details {
            Some(details) => write!(f, "{:?}: {}", self.kind, details),
            None => write!(f, "{:?}", self.kind),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source
            .as_deref()
            .map(|e| e as &(dyn StdError + 'static))
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::new(ErrorKind::DbError, value.to_string(), value)
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self::new(ErrorKind::ParseError, value.to_string(), value)
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::new(ErrorKind::IOError, value.to_string(), value)
    }
}

impl From<barcoders::error::Error> for Error {
    fn from(value: barcoders::error::Error) -> Self {
        Self::new(ErrorKind::PrintError, value.to_string(), value)
    }
}

impl From<printpdf::errors::Error> for Error {
    fn from(value: printpdf::errors::Error) -> Self {
        Self::new(ErrorKind::PrintError, value.to_string(), value)
    }
}

impl From<image_crate::error::ImageError> for Error {
    fn from(value: image_crate::error::ImageError) -> Self {
        Self::new(ErrorKind::PrintError, value.to_string(), value)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(value: serde_yaml::Error) -> Self {
        Self::new(ErrorKind::ConfigParseError, value.to_string(), value)
    }
}

impl From<password_hash::Error> for Error {
    fn from(_value: password_hash::Error) -> Self {
        Self::with_kind(ErrorKind::InvalidPassword)
    }
}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        Self::new(ErrorKind::InternalServer, value.to_string(), value)
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self::new(ErrorKind::ReqwestError, value.to_string(), value)
    }
}
