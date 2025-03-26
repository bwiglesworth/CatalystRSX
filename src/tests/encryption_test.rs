use catalyst_rsx::db::encryption::{DataEncryption, KeyManager};
use anyhow::Result;
use std::time::Duration;

#[test]
fn test_data_encryption() -> Result<()> {
    let key = [0u8; 32];
    let encryption = DataEncryption::new(&key);

    let sensitive_data = "secret password123";
    let encrypted = encryption.encrypt(sensitive_data)?;
    let decrypted = encryption.decrypt(&encrypted)?;

    assert_ne!(encrypted, sensitive_data);
    assert_eq!(decrypted, sensitive_data);
    Ok(())
}

#[tokio::test]
async fn test_key_rotation_with_timestamp() -> Result<()> {
    let mut key_manager = KeyManager::new();
    let initial_version = key_manager.get_current_version();
    let initial_timestamp = key_manager.get_key_creation_time(initial_version).await.unwrap();
    
    tokio::time::sleep(Duration::from_secs(1)).await;
    key_manager.rotate_key().await?;
    
    let new_version = key_manager.get_current_version();
    let new_timestamp = key_manager.get_key_creation_time(new_version).await.unwrap();
    
    assert!(new_timestamp > initial_timestamp);
    Ok(())}