use argon2::{
    password_hash::{PasswordHasher, SaltString, PasswordHash, PasswordVerifier},
    Argon2, Algorithm, Version, Params
};
use rand::rngs::OsRng;
use sqlx::MySqlPool;

pub struct PasswordManager {
    argon2: Argon2<'static>,
    pool: MySqlPool,
}

impl PasswordManager {
    pub fn new(pool: MySqlPool) -> Self {
        Self {
            argon2: Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Params::new(65536, 3, 4, None).unwrap()
            ),
            pool
        }
    }

    pub fn hash_password(&self, password: &str) -> Result<String, argon2::password_hash::Error> {
        let salt = SaltString::generate(&mut OsRng);
        let hash = self.argon2.hash_password(password.as_bytes(), &salt)?;
        Ok(hash.to_string())
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| e.to_string())?;
        
        Ok(self.argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }

    pub async fn update_user_password(&self, user_id: &str, new_password: &str) -> Result<(), String> {
        let new_hash = self.hash_password(new_password)
            .map_err(|e| e.to_string())?;
            
        sqlx::query!(
            "UPDATE users SET password_hash = ?, password_changed_at = NOW() WHERE id = ?",
            new_hash,
            user_id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::MySqlPool;
    use dotenv::dotenv;

    #[tokio::test]
    async fn test_password_hash_and_verify() {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let pool = MySqlPool::connect(&database_url)
            .await
            .expect("Failed to connect to database");
            
        let password_manager = PasswordManager::new(pool);
        let test_password = "Test123!@#";
        
        let hash = password_manager.hash_password(test_password).unwrap();
        assert!(password_manager.verify_password(test_password, &hash).unwrap());
    }
}