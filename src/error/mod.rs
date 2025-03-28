use actix_web::{
    error::ResponseError,
    http::StatusCode,
};
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    InternalError(String),
    AuthenticationError(String),
    ValidationError(String),
    DatabaseError(String),
    NotFoundError(String)
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::InternalError(msg) => write!(f, "Internal server error: {}", msg),
            Self::AuthenticationError(msg) => write!(f, "Authentication failed: {}", msg),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Self::NotFoundError(msg) => write!(f, "Resource not found: {}", msg),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFoundError(_) => StatusCode::NOT_FOUND,
        }
    }
}