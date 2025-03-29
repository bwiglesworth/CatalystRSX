use sqlx::MySqlPool;
use crate::security::password::PasswordManager;

pub fn verify_password(pool: &MySqlPool, password: &str, phc_string: &str) -> bool {
    let password_manager = PasswordManager::new(pool.clone());
    password_manager.verify_password(password, phc_string)
        .unwrap_or(false)
}

pub fn hash_password(pool: &MySqlPool, password: &str) -> Result<String, String> {
    let password_manager = PasswordManager::new(pool.clone());
    password_manager.hash_password(password)
        .map_err(|e| e.to_string())
}