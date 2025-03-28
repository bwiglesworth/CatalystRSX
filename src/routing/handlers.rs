use actix_web::{web, HttpResponse, Responder};
use crate::error::AppError;
use dioxus::prelude::*;
use dioxus_ssr::render_lazy;
use crate::templates::pages::admin::login::login_page;

#[derive(Serialize, Deserialize)]
pub struct User {
    name: String,
    email: String,
}

pub async fn dashboard_handler() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body("Admin Dashboard"))
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
pub async fn admin_login_page() -> Result<HttpResponse, AppError> {
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(render_lazy(rsx! {
            login_page {}
        })))
    }