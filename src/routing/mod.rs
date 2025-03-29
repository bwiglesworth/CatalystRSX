use actix_web::{web, guard, Scope, Error};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use crate::auth::guard::SessionGuard;
use crate::middleware::security::SecurityHeadersMiddleware;
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
        .wrap(SecurityHeadersMiddleware::new())
        .service(web::scope("/v1")
            .route("/health", web::get().to(health_check))
            .service(web::scope("/users")
                .route("", web::post().to(create_user))
                .route("/{id}", web::get().to(get_user))
                .route("/{id}", web::put().to(update_user))
                .route("/{id}", web::delete().to(delete_user))
            )
        )
}

pub fn admin_routes() -> Scope<impl ServiceFactory<
    ServiceRequest,
    Config = (),
    Response = ServiceResponse,
    Error = Error,
    InitError = (),
>> {
    web::scope("/admin")
        .wrap(SecurityHeadersMiddleware::new())
        .route("/login", web::get().to(login_page))
        .route("/login", web::post().to(handlers::login))
        .route("/dashboard", web::get().to(handlers::dashboard))
}