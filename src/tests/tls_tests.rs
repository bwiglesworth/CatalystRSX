use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use crate::config::Config;
#[test]
fn test_certificate_verification() {
    let config = Config::from_env().expect("Config loading works");
    
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())
        .expect("SSL acceptor created");
        
    builder
        .set_private_key_file(&config.server.tls_key_path, SslFiletype::PEM)
        .expect("Private key loaded");
        
    builder
        .set_certificate_chain_file(&config.server.tls_cert_path)
        .expect("Certificate chain loaded");
        
    assert!(builder.check_private_key().is_ok());
}

#[test]
fn test_tls_configuration() {
    let config = Config::from_env().expect("Config loading works");
    assert!(!config.server.tls_cert_path.is_empty());
    assert!(!config.server.tls_key_path.is_empty());
}