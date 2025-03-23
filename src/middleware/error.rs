use actix_web::{
    http::StatusCode,
    middleware::{ErrorHandlers, ErrorHandlerResponse},
    web, Result,
    dev::ServiceResponse,
    body::{MessageBody, EitherBody, BoxBody},
};

pub fn error_handlers(cfg: &mut web::ServiceConfig) {
    let error_handlers = ErrorHandlers::new()
        .handler(StatusCode::INTERNAL_SERVER_ERROR, handle_error)
        .handler(StatusCode::NOT_FOUND, handle_error);
    
    cfg.service(web::scope("").wrap(error_handlers));
}

pub fn handle_error<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> 
where
    B: MessageBody + 'static,
{
    let response = get_error_response(res)?;
    Ok(ErrorHandlerResponse::Response(response))
}

fn get_error_response<B>(res: ServiceResponse<B>) -> Result<ServiceResponse<EitherBody<B>>> 
where
    B: MessageBody + 'static,
{
    Ok(res.map_body(|_, _| {
            EitherBody::Right {
                body: BoxBody::new("Error occurred")
            }    }))
}