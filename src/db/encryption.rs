use aes_gcm::{Aes256Gcm, Key, Nonce, KeyInit};
use aes_gcm::aead::Aead;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use anyhow::Result;
use std::collections::HashMap;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};
pub struct DataEncryption {
    cipher: Aes256Gcm,
}

pub struct KeyVersion {
    key: [u8; 32],
    created_at: DateTime<Utc>,
}

pub struct KeyManager {
    current_version: u32,
    keys: RwLock<HashMap<u32, KeyVersion>>,
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

impl KeyManager {
    pub fn new() -> Self {
        let mut keys = HashMap::new();
        keys.insert(1, KeyVersion {
            key: generate_key(),
            created_at: Utc::now(),
        });
        
        Self {
            current_version: 1,
            keys: RwLock::new(keys),
        }
    }

    pub async fn rotate_key(&mut self) -> Result<()> {
        let new_version = self.current_version + 1;
        let new_key = KeyVersion {
            key: generate_key(),
            created_at: Utc::now(),
        };
        
        let mut write_guard = self.keys.write().await;
        write_guard.insert(new_version, new_key);
        self.current_version = new_version;
        Ok(())
    }

    pub fn get_current_version(&self) -> u32 {
        self.current_version
    }

    pub async fn get_key(&self, version: u32) -> Option<[u8; 32]> {
        self.keys.read().await.get(&version).map(|kv| kv.key)
    }

    pub async fn get_key_creation_time(&self, version: u32) -> Option<DateTime<Utc>> {
        self.keys.read().await.get(&version).map(|kv| kv.created_at)
    }
}fn generate_key() -> [u8; 32] {
    use rand::RngCore;
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);
    key
}