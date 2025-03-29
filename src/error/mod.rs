use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseError(String),
    AuthError(String),
    ValidationError(String),
    InternalError(String),
    AuthenticationError(String),
    NotFoundError(String),
    SessionError(String)
}


impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            Self::AuthError(msg) => write!(f, "Auth error: {}", msg),
            Self::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            Self::InternalError(msg) => write!(f, "Internal server error: {}", msg),
            Self::AuthenticationError(msg) => write!(f, "Authentication failed: {}", msg),
            Self::NotFoundError(msg) => write!(f, "Resource not found: {}", msg),
            Self::SessionError(msg) => write!(f, "Session error: {}", msg),
        }
    }
}
impl ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .json(serde_json::json!({
                "error": self.to_string()
            }))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AuthError(_) => StatusCode::UNAUTHORIZED,
            Self::ValidationError(_) => StatusCode::BAD_REQUEST,
            Self::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            Self::NotFoundError(_) => StatusCode::NOT_FOUND,
            Self::SessionError(_) => StatusCode::UNAUTHORIZED,
        }
    }
}

impl From<actix_web::Error> for AppError {
    fn from(error: actix_web::Error) -> Self {
        AppError::SessionError(error.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}
