mod config;
mod error;
mod middleware;
mod tests;

use actix_web::{App, HttpServer, web, HttpResponse};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use crate::middleware::error::error_handlers;
use crate::error::AppError;

async fn test_error() -> Result<HttpResponse, AppError> {
    Err(AppError::ValidationError("Test error".to_string()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv::dotenv().ok();
    env_logger::init();
    
    let config = config::Config::from_env().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // SSL Configuration
    let mut builder = SslAcceptor::mozilla_modern(SslMethod::tls())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    builder.set_private_key_file(&config.server.tls_key_path, SslFiletype::PEM)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    builder.set_certificate_chain_file(&config.server.tls_cert_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    HttpServer::new(move || {
        App::new()
            .wrap(error_handlers())
            .service(web::resource("/test-error").route(web::get().to(test_error)))
    })
    .bind_openssl(
        format!("{}:{}", config.server.host, config.server.port),
        builder
    )?
    .run()
    .await
}