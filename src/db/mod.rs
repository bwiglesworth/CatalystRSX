use sqlx::mysql::{MySqlPool, MySqlConnectOptions};
use anyhow::Result;
use dotenv::dotenv;
use std::env;

pub type DbPool = MySqlPool;

pub async fn create_pool() -> Result<DbPool> {
    dotenv().ok();
    let db_pass = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");

    let options = MySqlConnectOptions::new()
        .host("127.0.0.1")
        .username("catalystrsx")
        .password(&db_pass)
        .database("catalystrsx")
        .ssl_mode(sqlx::mysql::MySqlSslMode::VerifyIdentity)
        .ssl_ca("/etc/ssl/mariadb/ca-cert.pem")
        .ssl_client_cert("/etc/ssl/mariadb/client-cert.pem")
        .ssl_client_key("/etc/ssl/mariadb/client-key.pem");

    let pool = MySqlPool::connect_with(options).await?;
    Ok(pool)}