use actix_web::{web, HttpResponse};
use dioxus::prelude::*;
use dioxus_ssr::render_lazy;
use crate::templates::pages::index::index_page;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/")
            .route(web::get().to(index_handler))
    );
}

async fn index_handler() -> HttpResponse {
    let rendered = render_lazy(rsx! {
        index_page {}
    });

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}