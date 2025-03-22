use actix_web::{
    error::ResponseError,
    http::StatusCode,
};
use std::fmt;

#[derive(Debug)]
#[allow(dead_code)]
pub enum AppError {
    InternalError,
    ValidationError(String),
    AuthenticationError,
    AuthorizationError,
    NotFoundError,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InternalError => write!(f, "Internal server error"),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::AuthenticationError => write!(f, "Authentication failed"),
            Self::AuthorizationError => write!(f, "Authorization failed"),
            Self::NotFoundError => write!(f, "Resource not found"),
        }
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::AuthenticationError => StatusCode::UNAUTHORIZED,
            Self::AuthorizationError => StatusCode::FORBIDDEN,
            Self::NotFoundError => StatusCode::NOT_FOUND,
        }
    }
}