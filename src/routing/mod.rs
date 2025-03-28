use actix_web::web;
use crate::auth::guard::SessionGuard;
use crate::auth::admin::{AdminGuard, admin_login};
use self::handlers::*;
pub mod handlers;


pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .wrap(SessionGuard)
            .service(
                web::scope("/users")
                    .route("", web::post().to(create_user))
                    .route("/{id}", web::get().to(get_user))
                    .route("/{id}", web::put().to(update_user))
                    .route("/{id}", web::delete().to(delete_user))
            )
    )
    .service(
        web::scope("/admin")
            .route("/login", web::get().to(admin_login_page))
            .route("/login", web::post().to(admin_login))
            .wrap(AdminGuard::new())
            .route("/dashboard", web::get().to(dashboard_handler))
    )
    .service(
        web::resource("/")
            .route(web::get().to(index_handler))
    );
}