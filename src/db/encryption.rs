use aes_gcm::{Aes256Gcm, Key, Nonce, KeyInit};
use aes_gcm::aead::Aead;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use anyhow::Result;
pub struct DataEncryption {
    cipher: Aes256Gcm,
}

impl DataEncryption {
    pub fn new(encryption_key: &[u8; 32]) -> Self {
        let key = Key::<Aes256Gcm>::from_slice(encryption_key);        let cipher = Aes256Gcm::new(key);
        Self { cipher }
    }

    pub fn encrypt(&self, data: &str) -> Result<String> {
        let nonce = Nonce::from_slice(b"unique nonce");
        let encrypted = self.cipher
            .encrypt(nonce, data.as_bytes())
            .map_err(|e| anyhow::anyhow!("Encryption failed: {}", e))?;
        
        Ok(BASE64.encode(encrypted))
    }

    pub fn decrypt(&self, encrypted_data: &str) -> Result<String> {
        let decoded = BASE64.decode(encrypted_data)?;
        let nonce = Nonce::from_slice(b"unique nonce");
        let decrypted = self.cipher
            .decrypt(nonce, decoded.as_slice())
            .map_err(|e| anyhow::anyhow!("Decryption failed: {}", e))?;
            
        String::from_utf8(decrypted)
            .map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))
    }
}
