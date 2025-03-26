use catalyst_rsx::db::{create_pool, query::SafeQuery};
use anyhow::Result;
use sqlx::FromRow;
#[derive(FromRow)]
struct TestUser {
    id: i32,
    name: String,
}
#[tokio::test]
async fn test_safe_query() -> Result<()> {
    let pool = create_pool().await?;
    
    // Drop existing table if it exists
    sqlx::query("DROP TABLE IF EXISTS test_users").execute(&pool).await?;
    
    // Create fresh test table
    sqlx::query(
        "CREATE TABLE test_users (
            id INT PRIMARY KEY,
            name VARCHAR(255)
        )"
    ).execute(&pool).await?;
    
    // Insert test data
    sqlx::query(
        "INSERT INTO test_users (id, name) VALUES (1, 'test_user')"
    ).execute(&pool).await?;
    
    // Test safe parameterized query
    let users: Vec<TestUser> = SafeQuery::new()
        .select(&["id", "name"])
        .from("test_users")
        .fetch_all(&pool)
        .await?;
        
    // Test both fields
    assert_eq!(users[0].id, 1);
    assert_eq!(users[0].name, "test_user");
    
    // Cleanup
    sqlx::query("DROP TABLE test_users").execute(&pool).await?;
    
    Ok(())}