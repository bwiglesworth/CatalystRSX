use actix_web::web::JsonConfig;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct UserInput {
    #[validate(length(min = 3, max = 50))]
    pub name: String,
    #[validate(email)]
    pub email: String,}

#[allow(dead_code)]
pub fn configure_validation() -> JsonConfig {
    JsonConfig::default()
        .limit(4096) // 4kb limit
        .error_handler(|err, _| {
            actix_web::error::InternalError::from_response(
                err,
                actix_web::HttpResponse::BadRequest().finish(),
            )
            .into()
        })
}
