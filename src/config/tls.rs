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
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenv::dotenv().ok();

        let server = ServerConfig {
            host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: env::var("SERVER_PORT").unwrap_or_else(|_| "443".to_string()).parse().context("Failed to parse SERVER_PORT")?,
            tls_cert_path: env::var("TLS_CERT_PATH").unwrap_or_else(|_| "/etc/ssl/certs/devops.nceak.cert".to_string()),
            tls_key_path: env::var("TLS_KEY_PATH").unwrap_or_else(|_| "/etc/ssl/private/devops.nceak.key".to_string()),
        };

        Ok(Config { server })
    }
}