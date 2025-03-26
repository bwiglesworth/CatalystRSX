#[cfg(test)]
mod tests {
    use catalyst_rsx::security::password::{PasswordManager, PasswordPolicy};
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
}
