use once_cell::sync::OnceCell;
use sqlx::MySqlPool;

pub type DbPool = MySqlPool;

static POOL: OnceCell<MySqlPool> = OnceCell::new();

pub fn init_pool(pool: MySqlPool) {
    POOL.set(pool).expect("Failed to initialize pool");
}

pub fn get_pool() -> &'static MySqlPool {
    POOL.get().expect("Database pool not initialized")
}

pub async fn create_pool() -> MySqlPool {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
        
    MySqlPool::connect(&database_url)
        .await
        .expect("Failed to create database pool")
}