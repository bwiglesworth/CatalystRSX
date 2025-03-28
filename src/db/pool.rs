use once_cell::sync::OnceCell;
use sqlx::MySqlPool;

static POOL: OnceCell<MySqlPool> = OnceCell::new();

pub fn init_pool(pool: MySqlPool) {
    POOL.set(pool).expect("Failed to initialize pool");
}

pub fn get_pool() -> &'static MySqlPool {
    POOL.get().expect("Database pool not initialized")
}