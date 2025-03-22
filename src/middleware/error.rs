use actix_web::{
    dev::ServiceResponse,
    middleware::{ErrorHandlerResponse, ErrorHandlers},
    http::StatusCode,
    body::{BoxBody, MessageBody},
};
use crate::error::AppError;

pub fn error_handlers() -> ErrorHandlers<BoxBody> {
    ErrorHandlers::new()
        .handler(StatusCode::INTERNAL_SERVER_ERROR, |res| {
            let res = get_error_response(res, AppError::InternalError);
            Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
        })
        .handler(StatusCode::BAD_REQUEST, |res| {
            let res = get_error_response(res, AppError::ValidationError("Bad Request".to_string()));
            Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
        })
}

fn get_error_response<B: MessageBody + 'static>(res: ServiceResponse<B>, _error: AppError) -> ServiceResponse<BoxBody> {
    res.map_into_boxed_body()}