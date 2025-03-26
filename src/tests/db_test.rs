use catalyst_rsx::db::*;
use anyhow::Result;
use sqlx::Row;

#[tokio::test]
async fn test_database_connectivity() -> Result<()> {
    let pool = create_pool().await?;
    
    let row = sqlx::query("SHOW STATUS LIKE 'Ssl_cipher'")
        .fetch_one(&pool)
        .await?;
        
    let cipher: String = row.get(1);
    assert!(!cipher.is_empty());
    Ok(())
}