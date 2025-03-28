use actix_web::{web, Scope, Error};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use crate::auth::guard::SessionGuard;
use crate::auth::admin::{AdminGuard, admin_login, AdminLoginData};use self::handlers::*;
pub mod handlers;

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
        .service(web::scope("/admin")
            .route("", web::get().to(handlers::admin_login_page))
            .route("/login", web::post().to(admin_login))
            .route("/dashboard", web::get().to(dashboard_handler))
            .wrap(AdminGuard::new())
        )}