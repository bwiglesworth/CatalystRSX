use actix_web::{
    middleware::{ErrorHandlers, ErrorHandlerResponse},
    http::StatusCode,
    dev::ServiceResponse,
    body::{MessageBody, EitherBody},
    web::Bytes,
};
pub fn error_handlers(cfg: &mut actix_web::web::ServiceConfig) {
    let error_handlers = ErrorHandlers::<Bytes>::new()
        .handler(StatusCode::INTERNAL_SERVER_ERROR, handle_error)
        .handler(StatusCode::BAD_REQUEST, handle_error)
        .handler(StatusCode::UNAUTHORIZED, handle_error)
        .handler(StatusCode::FORBIDDEN, handle_error)
        .handler(StatusCode::NOT_FOUND, handle_error);

    cfg.app_data(error_handlers);
}

fn handle_error<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>, actix_web::Error>
where
    B: MessageBody + 'static,
{
    let response = get_error_response(res)?;
    Ok(ErrorHandlerResponse::Response(response))
}

fn get_error_response<B>(res: ServiceResponse<B>) -> Result<ServiceResponse<EitherBody<B>>, actix_web::Error>
where
    B: MessageBody + 'static,
{
    Ok(res.map_into_left_body())
}