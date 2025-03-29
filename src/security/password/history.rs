use std::collections::VecDeque;
use time::OffsetDateTime;
use super::PasswordExpiration;
pub struct PasswordHistory {
    history: VecDeque<PasswordEntry>,
    max_entries: usize,
}

use sqlx::MySqlPool;

struct PasswordEntry {
    hash: String,
    timestamp: OffsetDateTime,
}

impl PasswordHistory {
    pub fn new(max_entries: usize) -> Self {
        PasswordHistory {
            history: VecDeque::with_capacity(max_entries),
            max_entries,
        }
    }

    pub fn add_password(&mut self, pool: &MySqlPool, password: &str) -> Result<(), String> {
        let password_manager = super::PasswordManager::new(pool.clone());
        let hash = password_manager.hash_password(password)
            .map_err(|e| e.to_string())?;
            
        if self.history.len() >= self.max_entries {
            self.history.pop_back();
        }
        
        self.history.push_front(PasswordEntry {
            hash,
            timestamp: OffsetDateTime::now_utc()
        });
        
        Ok(())
    }

    pub fn contains(&self, pool: &MySqlPool, password: &str) -> bool {
        let password_manager = super::PasswordManager::new(pool.clone());
        self.history.iter().any(|entry| {
            password_manager.verify_password(password, &entry.hash)
                .unwrap_or(false)
        })
    }

    pub fn get_password_age(&self, hash: &str) -> Option<OffsetDateTime> {
        self.history
            .iter()
            .find(|entry| entry.hash == hash)
            .map(|entry| entry.timestamp)
    }

    pub fn check_expiration(&self, hash: &str, expiration: &PasswordExpiration) -> bool {
        self.get_password_age(hash)
            .map(|timestamp| expiration.is_expired(timestamp))
            .unwrap_or(false)
    }
}