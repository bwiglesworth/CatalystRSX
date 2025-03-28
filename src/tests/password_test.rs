#[cfg(test)]
mod tests {
    use crate::security::password::{PasswordManager, PasswordPolicy, PasswordHistory, PasswordExpiration};
    use time::{Duration, OffsetDateTime};

    #[test]
    fn test_password_hashing() {
        let password_manager = PasswordManager::new();
        let password = "MySecurePassword123!";
        let hash = password_manager.hash_password(password).unwrap();
        
        assert!(hash.starts_with("$argon2id$"));
        assert!(password_manager.verify_password(password, &hash).unwrap());
    }

    #[test]
    fn test_password_verification_fails() {
        let password_manager = PasswordManager::new();
        let password = "MySecurePassword123!";
        let wrong_password = "WrongPassword123!";
        let hash = password_manager.hash_password(password).unwrap();
        
        assert!(!password_manager.verify_password(wrong_password, &hash).unwrap());
    }

    #[test]
    fn test_password_policy_valid() {
        let policy = PasswordPolicy::new();
        let valid_password = "SecurePass123!@";
        assert!(policy.validate(valid_password).is_ok());
    }

    #[test]
    fn test_password_policy_too_short() {
        let policy = PasswordPolicy::new();
        let short_password = "Short1!";
        let result = policy.validate(short_password);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains(&"Password must be at least 12 characters".to_string()));
    }

    #[test]
    fn test_password_policy_missing_requirements() {
        let policy = PasswordPolicy::new();
        let weak_password = "weakpassword";
        let errors = policy.validate(weak_password).unwrap_err();
        
        assert!(errors.contains(&"Password must contain at least one uppercase letter".to_string()));
        assert!(errors.contains(&"Password must contain at least one number".to_string()));
        assert!(errors.contains(&"Password must contain at least one special character".to_string()));
    }

    #[test]
    fn test_password_history_tracking() {
        let password_manager = PasswordManager::new();
        let mut history = PasswordHistory::new(3);
        
        let password1 = "SecurePass123!@";
        let hash1 = password_manager.hash_password(password1).unwrap();
        history.add(hash1);
        
        assert!(history.contains(password1));
    }

    #[test]
    fn test_password_history_limit() {
        let password_manager = PasswordManager::new();
        let mut history = PasswordHistory::new(2);
        
        let passwords = ["Pass1!@#", "Pass2!@#", "Pass3!@#"];
        
        for pass in passwords.iter() {
            let hash = password_manager.hash_password(pass).unwrap();
            history.add(hash);
        }
        
        assert!(!history.contains(passwords[0])); // Oldest password should be removed
        assert!(history.contains(passwords[1]));
        assert!(history.contains(passwords[2]));
    }

    #[test]
    fn test_password_not_expired() {
        let expiration = PasswordExpiration::new(90);
        let timestamp = OffsetDateTime::now_utc();
        assert!(!expiration.is_expired(timestamp));
    }

    #[test]
    fn test_password_is_expired() {
        let expiration = PasswordExpiration::new(90);
        let old_timestamp = OffsetDateTime::now_utc() - Duration::days(91);
        assert!(expiration.is_expired(old_timestamp));
    }

    #[test]
    fn test_password_expiration_edge_case() {
        let expiration = PasswordExpiration::new(90);
        let edge_timestamp = OffsetDateTime::now_utc() - Duration::days(90);
        assert!(!expiration.is_expired(edge_timestamp));
    }
}
