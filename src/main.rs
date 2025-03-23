mod auth;
mod config;
mod error;
//mod middleware;
mod tests;
use actix_web::{App, HttpServer};
use actix_web::middleware::Logger;
use actix_governor::{Governor, GovernorConfigBuilder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use crate::auth::session::configure_session;
use catalyst_rsx::routes::configure_routes;
//use catalyst_rsx::middleware::error::error_handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();

    let config = config::Config::from_env().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_certificate_file(&config.server.tls_cert_path, SslFiletype::PEM)?;
    builder.set_private_key_file(&config.server.tls_key_path, SslFiletype::PEM)?;

    HttpServer::new(move || {
        App::new()
            .wrap(Governor::new(&governor_conf))
            .wrap(configure_session())
            .wrap(Logger::default())
            .configure(configure_routes)
    })
    .bind_openssl(&format!("{}:{}", config.server.host, config.server.port), builder)?
    .run()
    .await
}