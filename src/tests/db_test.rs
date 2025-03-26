use catalyst_rsx::db::*;
use anyhow::Result;
use sqlx::Row;

#[tokio::test]
async fn test_database_connectivity() -> Result<()> {
    let pool = create_pool().await?;
    
    // Check if SSL is being used
    let row = sqlx::query("SHOW STATUS LIKE 'Ssl_cipher'")
        .fetch_one(&pool)
        .await?;
        
    let cipher: String = row.get(1);
    println!("SSL Cipher in use: {}", cipher);
    assert!(!cipher.is_empty(), "Connection should be encrypted");

    // Additional SSL verification
    let version: String = sqlx::query("SHOW STATUS LIKE 'Ssl_version'")
        .fetch_one(&pool)
        .await?
        .get(1);
    println!("SSL Version: {}", version);
    
    Ok(())
}