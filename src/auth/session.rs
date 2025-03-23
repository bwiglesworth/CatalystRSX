use actix_session::SessionMiddleware;
use actix_session::storage::CookieSessionStore;
use actix_web::cookie::{Key, SameSite};
use actix_web::cookie::time::Duration;

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