use actix_web::{web, HttpResponse};
use dioxus::prelude::*;
use dioxus_ssr::render_lazy;
use crate::templates::pages::dashboard::dashboard_page;
use crate::auth::admin::AdminGuard;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            .wrap(AdminGuard)
            .route("/dashboard", web::get().to(dashboard_handler))
    );
}

async fn dashboard_handler() -> HttpResponse {
    let rendered = render_lazy(rsx! {
        dashboard_page {}
    });

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)}