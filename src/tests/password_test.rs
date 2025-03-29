#[cfg(test)]
mod tests {
    use sqlx::MySqlPool;
    use crate::security::password::PasswordManager;
    use crate::security::password::history::PasswordHistory;
    //use crate::security::password::{policy, expiration};
    use dotenv::dotenv;

    async fn setup_test_pool() -> Result<MySqlPool, String> {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .map_err(|e| format!("Failed to get DATABASE_URL from .env: {}", e))?;
            
        MySqlPool::connect(&database_url)
            .await
            .map_err(|e| format!("Database connection error: {:?}", e))
    }

    #[tokio::test]
    async fn test_password_hash_verify() {
        let pool = setup_test_pool()
            .await
            .expect("Test pool setup failed - check .env DATABASE_URL");
            
        let password_manager = PasswordManager::new(pool);
        let test_password = "Test123!@#";
        
        let hash = password_manager.hash_password(test_password)
            .expect("Password hashing failed");
            
        assert!(password_manager.verify_password(test_password, &hash)
            .expect("Password verification failed"));
    }

    #[tokio::test]
    async fn test_password_history() {
        let pool = setup_test_pool().await.expect("Failed to setup test pool");
        let mut history = PasswordHistory::new(3);
        
        for password in ["pass1", "pass2", "pass3"] {
            history.add_password(&pool, password).unwrap();
        }

        assert!(history.contains(&pool, "pass3"));
        assert!(!history.contains(&pool, "nonexistent"));
    }

    #[cfg(test)]
    mod tests {
        use crate::security::password::policy;

        #[test]
        fn test_password_policy_compliance() {
            let policy = policy::PasswordPolicy::new();
            
            // This should pass all requirements
            assert!(policy.validate("Strong1!Password").is_ok());
            
            // These should fail various requirements
            assert!(policy.validate("weak").is_err());
            assert!(policy.validate("nocapitals123!").is_err());
            assert!(policy.validate("NOSMALL123!").is_err());
            assert!(policy.validate("NoSpecial123").is_err());
            assert!(policy.validate("NoNumbers!").is_err());
        }

        #[test]
        fn test_password_edge_cases() {
            let policy = policy::PasswordPolicy::new();
            
            assert!(policy.validate("").is_err());
            assert!(policy.validate("🔒").is_err());
            assert!(policy.validate(&"a".repeat(129)).is_err());
        }
}}    