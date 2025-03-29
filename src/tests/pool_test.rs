#[cfg(test)]
mod tests {
    use crate::db::{self, pool};

    #[tokio::test]
    async fn test_pool_initialization() {
        let db_pool = db::create_pool().await.expect("Failed to create pool");
        pool::init_pool(db_pool);
        
        let pool = pool::get_pool();
        assert!(pool.is_valid().await.is_ok());
    }
}
