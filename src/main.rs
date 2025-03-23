mod auth;
mod config;
mod error;
mod middleware;
mod tests;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_files as fs;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use crate::middleware::error::error_handlers;
use crate::auth::session::configure_session;
use crate::auth::guard::SessionGuard;#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv::dotenv().ok();
    env_logger::init();
    
    // Configure rate limiting
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();

    // Load configuration
    let config = config::Config::from_env().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Set up SSL
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    builder.set_private_key_file(&config.server.tls_key_path, SslFiletype::PEM)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    builder.set_certificate_chain_file(&config.server.tls_cert_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Start HTTP server with all middleware
    HttpServer::new(move || {
        App::new()
    .wrap(Logger::default())
    .wrap(configure_session())
    .wrap(Governor::new(&governor_conf))
    .configure(error_handlers)
    .service(
        web::scope("/api")
            .wrap(SessionGuard)
            )            .service(fs::Files::new("/", "./static").index_file("index.html"))
    })
    .bind_openssl(&format!("{}:{}", config.server.host, config.server.port), builder)?
    .run()
    .await}