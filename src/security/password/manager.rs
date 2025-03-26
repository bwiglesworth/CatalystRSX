use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2
};

pub struct PasswordManager {
    argon2: Argon2<'static>
}

impl PasswordManager {
    pub fn new() -> Self {
        PasswordManager {
            argon2: Argon2::default()
        }
    }

    pub fn hash_password(&self, password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        self.argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| e.to_string())
    }

    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String> {
        let parsed_hash = PasswordHash::new(hash).map_err(|e| e.to_string())?;
        Ok(self.argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}
