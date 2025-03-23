use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    id: String,
    name: String,
    email: String,
}

pub async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("Service is healthy")
}

pub async fn create_user(user: web::Json<User>) -> impl Responder {
    HttpResponse::Created().json(user.0)
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
