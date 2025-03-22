use crate::config;
use openssl::ssl::{SslAcceptor, SslMethod, SslVersion, SslFiletype};
#[test]
fn test_tls_version() {
    let mut builder = SslAcceptor::mozilla_modern(SslMethod::tls()).expect("SSL acceptor created");
    
    // Test minimum TLS version - mozilla_modern defaults to TLS 1.2
    let version = builder.min_proto_version().unwrap();
    assert!(matches!(version, SslVersion::TLS1_2));
}

#[test]
fn test_cipher_suite() {
    let mut builder = SslAcceptor::mozilla_modern(SslMethod::tls()).expect("SSL acceptor created");
    
    // Test setting strong cipher suite
    builder.set_cipher_list("ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384")
        .expect("Cipher suite set successfully");
}

#[test]
fn test_certificate_verification() {
    let config = config::Config::from_env().expect("Config loading works");
    let mut builder = SslAcceptor::mozilla_modern(SslMethod::tls()).expect("SSL acceptor created");
    
    builder.set_private_key_file(&config.server.tls_key_path, SslFiletype::PEM)
        .expect("Private key loaded");
    builder.set_certificate_chain_file(&config.server.tls_cert_path)
        .expect("Certificate chain loaded");
    
    // Test peer verification mode
    builder.set_verify(openssl::ssl::SslVerifyMode::PEER);
    assert!(builder.check_private_key().is_ok());
}

#[test]
fn test_ocsp_configuration() {
    let config = config::Config::from_env().expect("Config loading works");
    if !config.server.ocsp_responder.is_empty() {
        let mut builder = SslAcceptor::mozilla_modern(SslMethod::tls()).expect("SSL acceptor created");
        builder.set_verify(openssl::ssl::SslVerifyMode::PEER);
        assert!(builder.check_private_key().is_ok());
    }
}