use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde_json::json;

/// This enum is used to categorize errors that occur in the application and provide a consistent
/// way to handle and report errors. Each error type has a corresponding message that can be used.
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Clone)]
pub enum ErrorType {
    ParseError,
    ThirdPartyError,
    InternalError,
    InvalidInput,
    NotFound,
    TimeoutError,
    HttpError,
    DatabaseError,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: String,
}

impl Error {
    pub fn kind(&self) -> ErrorType {
        self.error_type.clone()
    }
}


impl IntoResponse for Error {
    fn into_response(self) -> Response {
        // Map your ErrorType to an HTTP status code
        let status = match self.error_type {
            ErrorType::ParseError => StatusCode::BAD_REQUEST,
            ErrorType::InvalidInput => StatusCode::BAD_REQUEST,
            ErrorType::NotFound => StatusCode::NOT_FOUND,
            ErrorType::TimeoutError => StatusCode::REQUEST_TIMEOUT,
            ErrorType::HttpError => StatusCode::BAD_REQUEST,
            ErrorType::DatabaseError | ErrorType::InternalError | ErrorType::ThirdPartyError => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = json!({
            "error_type": format!("{:?}", self.error_type),
            "message": self.message,
        });

        (status, axum::Json(body)).into_response()
    }
}

impl Error {
    pub fn internal(p0: &str) -> Self {
        use std::io::Write;
        let _ = writeln!(std::io::stderr(), "internal error: {p0}");
        Self::new(ErrorType::InternalError, p0)
    }
}

impl Error {
    pub fn new(error_type: ErrorType, message: &str) -> Self {
        Error {
            error_type,
            message: message.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.error_type, self.message)
    }
}

impl From<uuid::Error> for Error {
    fn from(err: uuid::Error) -> Self {
        Error {
            error_type: ErrorType::ParseError,
            message: format!("{err:?}")
        }
    }
}

impl From<axum::http::Error> for Error {
    fn from(err: axum::http::Error) -> Self {
        Error {
            error_type: ErrorType::HttpError,
            message: format!("{err:?}")
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl From<chrono::ParseError> for Error {
    fn from(error: chrono::ParseError) -> Self {
        Error {
            error_type: ErrorType::ParseError,
            message: error.to_string(),
        }
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Error {
            error_type: ErrorType::ParseError,
            message: error.to_string(),
        }
    }
}
impl From<Box<dyn std::error::Error + std::marker::Send + Sync>> for Error {
    fn from(error: Box<dyn std::error::Error + std::marker::Send + Sync>) -> Self {
        Error {
            error_type: ErrorType::InternalError,
            message: error.to_string(),
        }
    }
}
