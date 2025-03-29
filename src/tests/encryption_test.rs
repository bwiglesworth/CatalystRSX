#[cfg(test)]
mod tests {
    use crate::db;

    #[test]
    fn test_encryption() {
        let key = [0u8; 32];
        let encryption = db::encryption::DataEncryption::new(&key);
        
        let original = "test data";
        let encrypted = encryption.encrypt(original).unwrap();
        let decrypted = encryption.decrypt(&encrypted).unwrap();
        
        assert_eq!(original, decrypted);
    }

    #[tokio::test]
    async fn test_key_manager() {
        let mut key_manager = db::encryption::KeyManager::new();
        assert_eq!(key_manager.get_current_version(), 1);
        
        key_manager.rotate_key().await.unwrap();
        assert_eq!(key_manager.get_current_version(), 2);
        
        let key = key_manager.get_key(2).await;
        assert!(key.is_some());
    }
}