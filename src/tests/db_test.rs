use crate::db;
use anyhow::Result;
use sqlx::Row;
use std::time::Duration;

#[tokio::test]
async fn test_database_connectivity() -> Result<()> {
    let pool = db::create_pool().await?;
    let row = sqlx::query("SHOW STATUS LIKE 'Ssl_cipher'")
        .fetch_one(&pool)
        .await?;
    let cipher: String = row.get(1);
    assert!(!cipher.is_empty());
    Ok(())
}

#[tokio::test]
async fn test_pool_configuration() -> Result<()> {
    let pool = db::create_pool().await?;
    // Test multiple concurrent connections
    let mut handles = vec![];
    for _ in 0..10 {
        let pool = pool.clone();
        handles.push(tokio::spawn(async move {
            let _conn = pool.acquire().await.unwrap();
            tokio::time::sleep(Duration::from_millis(100)).await;
        }));
    }
    futures::future::join_all(handles).await;
    Ok(())
}