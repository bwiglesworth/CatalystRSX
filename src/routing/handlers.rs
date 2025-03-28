use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use dioxus::prelude::*;
use dioxus_ssr::render_lazy;
use crate::error::AppError;
use crate::templates::pages::index::index_page;
use crate::templates::pages::admin::login::login_page;

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
}

pub async fn admin_login_page() -> impl Responder {
    let rendered = render_lazy(rsx! {
        login_page {}
    });
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn index_handler() -> impl Responder {
    let rendered = render_lazy(rsx! {
        index_page {}
    });
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn dashboard_handler() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok().body("Dashboard"))
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Health check OK")
}

pub async fn create_user(user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(user.0)
}

pub async fn get_user(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(format!("Get user {}", id))
}

pub async fn update_user(id: web::Path<String>, user: web::Json<User>) -> impl Responder {
    HttpResponse::Ok().json(format!("Updated user {} with name {}", id, user.name))
}

pub async fn delete_user(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(format!("Deleted user {}", id))
}

pub async fn login_handler() -> HttpResponse {
    let rendered = render_lazy(rsx! {
        login_page {}
    });

    HttpResponse::Ok()
        .content_type("text/html")
        .body(rendered)
}
