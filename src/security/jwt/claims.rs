use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // Subject (user id)
    pub exp: usize,   // Expiration time
    pub iat: usize,   // Issued at
    pub role: String, // User role
}

impl Claims {
    pub fn new(user_id: String, role: String, exp_mins: usize) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
            
        Self {
            sub: user_id,
            exp: now + (exp_mins * 60),
            iat: now,
            role,
        }
    }
}
