use actix_web::{web, HttpResponse};
use dioxus_ssr::render_lazy;
use crate::templates::pages::index::index_page;
use crate::auth::admin::{AdminGuard, admin_login};
use self::handlers::*;
pub mod handlers;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .wrap(SessionGuard)
            .service(api_routes())
    )
    .service(
        web::scope("/admin")
            .route("/login", web::get().to(handlers::admin_login_page))
            .route("/login", web::post().to(admin_login))
            .wrap(AdminGuard::new())
            .route("/dashboard", web::get().to(dashboard_handler))
    )
    .service(
        web::resource("/")
            .route(web::get().to(index_handler))
    );
}        )}