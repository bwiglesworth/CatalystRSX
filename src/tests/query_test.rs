#[cfg(test)]
mod tests {
    use crate::db::{self, pool};
    use crate::db::query::SafeQuery;
    use sqlx::FromRow;
    use serde::{Serialize, Deserialize};

    #[derive(Debug, FromRow, Serialize, Deserialize)]
    struct TestUser {
        id: String,
        username: String,  // Changed from 'name' to 'username'
    }

    #[tokio::test]
    async fn test_query_builder() {
        let db_pool = db::create_pool().await.expect("Failed to create pool");
        pool::init_pool(db_pool);
        
        let pool = pool::get_pool();
        let query = SafeQuery::<TestUser>::new()
            .select(&["id", "username"])  // Updated column name
            .from("users");
        let _results = query.fetch_all(pool).await.expect("Failed to fetch users");
    }
}