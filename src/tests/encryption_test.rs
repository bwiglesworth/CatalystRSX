use catalyst_rsx::db::encryption::DataEncryption;
use anyhow::Result;

#[test]
fn test_data_encryption() -> Result<()> {
    let key = [0u8; 32]; // In production, use a secure key management system
    let encryption = DataEncryption::new(&key);

    let sensitive_data = "secret password123";
    let encrypted = encryption.encrypt(sensitive_data)?;
    let decrypted = encryption.decrypt(&encrypted)?;

    assert_ne!(encrypted, sensitive_data);
    assert_eq!(decrypted, sensitive_data);
    Ok(())
}
