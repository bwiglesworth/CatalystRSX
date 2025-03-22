mod config;
mod validation;
#[cfg(test)]
mod tests;
use actix_web::{
    App, 
    HttpServer, 
    middleware::Logger,
    middleware::DefaultHeaders
};
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_files as fs;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod, SslVersion};
use openssl::x509::store::X509StoreBuilder;
use openssl::x509::verify::X509VerifyFlags;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv::dotenv().ok();
    env_logger::init();
    
    // Load configuration
    let config = config::Config::from_env().map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Set up SSL with modern configuration
    let mut builder = SslAcceptor::mozilla_modern(SslMethod::tls())
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    // Set TLS version based on configuration
    let min_version = match config.server.min_tls_version.as_str() {
        "TLS1.3" => SslVersion::TLS1_3,
        "TLS1.2" => SslVersion::TLS1_2,
        _ => SslVersion::TLS1_3,
    };
    
    builder.set_min_proto_version(Some(min_version))
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    builder.set_private_key_file(&config.server.tls_key_path, SslFiletype::PEM)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    
    builder.set_certificate_chain_file(&config.server.tls_cert_path)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Configure cipher suites
    builder.set_cipher_list(&config.server.cipher_suite)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // Configure certificate verification
    if !config.server.ocsp_responder.is_empty() {
        let mut cert_store = X509StoreBuilder::new()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        cert_store.set_flags(X509VerifyFlags::CRL_CHECK)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
            
        builder.set_verify_callback(openssl::ssl::SslVerifyMode::PEER, move |_, ctx| {
            ctx.current_cert().is_some()
        });    }    log::info!("Starting server at https://{}:{}", config.server.host, config.server.port);    HttpServer::new(move || {
        let governor_conf = GovernorConfigBuilder::default()
            .per_second(100)
            .burst_size(5)
            .finish()
            .unwrap();

        App::new()
            .wrap(Logger::default())
            .wrap(Governor::new(&governor_conf))
            .wrap(
                DefaultHeaders::new()
                    .add(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
                    .add(("X-Frame-Options", "DENY"))
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("Content-Security-Policy", "default-src 'self'"))
                    .add(("X-XSS-Protection", "1; mode=block"))
                    .add(("Referrer-Policy", "strict-origin-when-cross-origin"))
            )
            .service(fs::Files::new("/", "./static").index_file("index.html"))    })    .bind_openssl(
        format!("{}:{}", config.server.host, config.server.port),
        builder,
    )?
    .run()
    .await
}