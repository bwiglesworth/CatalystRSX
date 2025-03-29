use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::{Key, SameSite};
use actix_web::cookie::time::Duration;
use actix_session::Session;
use crate::models::user::User;
use crate::error::AppError;

pub fn configure_session() -> SessionMiddleware<CookieSessionStore> {
    let secret_key = Key::generate();
    
    SessionMiddleware::builder(
        CookieSessionStore::default(),
        secret_key
    )
    .cookie_secure(true)
    .cookie_http_only(true)
    .cookie_same_site(SameSite::Strict)
    .session_lifecycle(
        actix_session::config::PersistentSession::default()
            .session_ttl(Duration::hours(1))
    )
    .build()
}

pub async fn create_session(session: &Session, user: &User) -> Result<(), actix_web::Error> {
    session.insert("user_id", &user.id)?;
    session.insert("username", &user.username)?;
    session.insert("role", &user.role)?;
    
    // Update last login timestamp
    User::update_last_login(&user.id).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Reset failed login attempts
    User::reset_failed_attempts(&user.id).await.map_err(|e| AppError::DatabaseError(e.to_string()))?;
    
    // Set session timeout
    session.insert("expires_at", time::OffsetDateTime::now_utc() + Duration::hours(2))?;    Ok(())
}

pub fn clear_session(session: &Session) {
    session.purge();
}

pub fn is_authenticated(session: &Session) -> bool {
    session.get::<String>("user_id").unwrap_or(None).is_some()
}