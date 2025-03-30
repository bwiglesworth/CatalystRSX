use sqlx::mysql::{MySqlPool, MySqlConnectOptions};
use sqlx::pool::PoolOptions;
use sqlx::MySql;
use anyhow::Result;
use dotenv::dotenv;
use std::env;
use std::time::Duration;

pub type DbPool = MySqlPool;

pub mod pool;
pub mod query;
pub mod encryption;
pub async fn create_pool() -> Result<DbPool> {
    dotenv().ok();
    let db_pass = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");

    let options = MySqlConnectOptions::new()
        .host("127.0.0.1")
        .username("catalystrsx")
        .password(&db_pass)
        .database("catalystrsx")
        .ssl_mode(sqlx::mysql::MySqlSslMode::Required);

    let pool = PoolOptions::<MySql>::new()
        .max_connections(32)
        .min_connections(5)
        .max_lifetime(Duration::from_secs(3600))
        .idle_timeout(Duration::from_secs(600))
        .connect_with(options)
        .await?;

    Ok(pool)
}