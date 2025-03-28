use actix_web::{web, Scope, Error};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use crate::auth::guard::SessionGuard;
mod handlers;
use handlers::*;

pub fn api_routes() -> Scope<impl ServiceFactory<
    ServiceRequest,
    Config = (),
    Response = ServiceResponse,
    Error = Error,
    InitError = (),
>> {
    web::scope("/api")
        .wrap(SessionGuard)
        .service(web::scope("/v1")
            .route("/health", web::get().to(health_check))
            .service(web::scope("/users")
                .route("", web::post().to(create_user))
                .route("/{id}", web::get().to(get_user))
                .route("/{id}", web::put().to(update_user))
                .route("/{id}", web::delete().to(delete_user))
            )
        )
}}