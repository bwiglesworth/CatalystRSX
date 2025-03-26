use std::collections::VecDeque;
use time::OffsetDateTime;
use super::PasswordExpiration;
pub struct PasswordHistory {
    history: VecDeque<PasswordEntry>,
    max_entries: usize,
}

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

    pub fn add(&mut self, hash: String) {
        let entry = PasswordEntry {
            hash,
            timestamp: OffsetDateTime::now_utc(),
        };

        if self.history.len() >= self.max_entries {
            self.history.pop_back();
        }
        self.history.push_front(entry);
    }

    pub fn contains(&self, password: &str) -> bool {
        let password_manager = super::PasswordManager::new();
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