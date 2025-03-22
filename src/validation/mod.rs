use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;
use crate::error::AppError;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserInput {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    #[validate(email)]
    pub email: String,
}

pub fn configure_validation(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/validate")
            .route(web::post().to(validate_input))
    );
}

async fn validate_input(input: web::Json<UserInput>) -> Result<HttpResponse, AppError> {
    input.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    Ok(HttpResponse::Ok().finish())}