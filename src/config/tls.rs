use serde::Deserialize;
use std::env;
use anyhow::{Result, Context};
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
}
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub tls_cert_path: String,
    pub tls_key_path: String,
    pub min_tls_version: String,
    pub cipher_suite: String,
    pub ocsp_responder: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let server = ServerConfig {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("SERVER_PORT").unwrap_or_else(|_| "443".to_string()).parse().context("Failed to parse SERVER_PORT")?,
            tls_cert_path: env::var("TLS_CERT_PATH").unwrap_or_else(|_| "/etc/ssl/certs/devops.nceak.cert".to_string()),
            tls_key_path: env::var("TLS_KEY_PATH").unwrap_or_else(|_| "/etc/ssl/private/devops.nceak.key".to_string()),
            min_tls_version: env::var("TLS_MIN_VERSION").unwrap_or_else(|_| "TLS1.3".to_string()),
            cipher_suite: env::var("TLS_CIPHER_SUITE").unwrap_or_else(|_| "ECDHE-ECDSA-AES256-GCM-SHA384:ECDHE-RSA-AES256-GCM-SHA384".to_string()),
            ocsp_responder: env::var("OCSP_RESPONDER_URL").unwrap_or_else(|_| "".to_string()),
        };

        Ok(Config { server })
    }
}