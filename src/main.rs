mod auth;
mod config;
mod error;
mod tests;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::middleware::Logger;
use actix_governor::{Governor, GovernorConfigBuilder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use catalyst_rsx::routing::api_routes;
use crate::auth::session::configure_session;
use crate::auth::admin::AdminGuard;
use catalyst_rsx::routing::handlers::dashboard_handler;

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
            .wrap(Logger::default())
            .wrap(Governor::new(&governor_conf))
            .wrap(configure_session())
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Welcome to CatalystRSX") }))
            .service(api_routes())
            .service(
                web::scope("/admin")
                    .wrap(AdminGuard::new())
                    .route("/dashboard", web::get().to(dashboard_handler))            )
    })
    .bind_openssl(&format!("{}:{}", config.server.host, config.server.port), builder)?
    .run()
    .await
}