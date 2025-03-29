use actix_web::{web, HttpResponse, Responder};
use serde::{Serialize, Deserialize};
use dioxus::prelude::*;
use dioxus_ssr::render_lazy;
use crate::error::AppError;
use crate::templates::pages::index::index_page;
use crate::templates::pages::admin::login::login_page;
use crate::templates::pages::admin::dashboard::dashboard_page;
use crate::security::password::PasswordManager;
use crate::models::user::User;
use crate::auth::session;
use crate::db::pool;

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    username: String,
    password: String,
}

pub async fn admin_login_page() -> impl Responder {
    let rendered = render_lazy(rsx! {
        login_page {}
    });
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

pub async fn admin_login(
    form: web::Form<LoginForm>,
    session: actix_session::Session,
) -> Result<HttpResponse, AppError> {
    println!("Login attempt for username: {}", form.username);
    let pool = pool::get_pool();
    let password_manager = PasswordManager::new(pool.clone());
    match User::find_by_username(&form.username, pool).await.map_err(AppError::from)? {
        Some(user) => {
            println!("User found: {:?}", user);
            match password_manager.verify_password(&form.password, &user.password_hash) {
                Ok(true) => {
                    session::create_session(&session, &user).await.map_err(AppError::from)?;
                    Ok(HttpResponse::Found()
                        .append_header(("Location", "/admin/dashboard"))
                        .finish())                },                _ => {
                    User::increment_failed_attempts(&user.id).await.map_err(AppError::from)?;
                    Ok(HttpResponse::Found()
                        .append_header(("Location", "/admin/login?error=invalid_credentials"))
                        .finish())
                }
            }
        },
        None => {
            Ok(HttpResponse::Found()
                .append_header(("Location", "/admin/login?error=user_not_found"))
                .finish())
        }
    }
}
pub async fn dashboard_handler(session: actix_session::Session) -> HttpResponse {
    match (session.get::<String>("user_id"), session.get::<String>("role")) {
        (Ok(Some(_)), Ok(Some(role))) => {
            let rendered = render_lazy(rsx! {
                dashboard_page {
                    username: "admin".to_string(),
                    role: role.trim_matches('"').to_string()
                }
            });
            HttpResponse::Ok()
                .content_type("text/html")
                .body(rendered)
        },
        _ => {
            HttpResponse::Found()
                .append_header(("Location", "/admin/login"))
                .finish()
        }
    }
}pub async fn index_handler() -> impl Responder {
    let rendered = render_lazy(rsx! {
        index_page {}
    });
    HttpResponse::Ok().content_type("text/html").body(rendered)
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
    HttpResponse::Ok().json(format!("Updated user {} with name {}", id, user.username))}

pub async fn delete_user(id: web::Path<String>) -> impl Responder {
    HttpResponse::Ok().json(format!("Deleted user {}", id))
}