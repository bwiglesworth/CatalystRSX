use serde::{Serialize, Deserialize};
use time::OffsetDateTime;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>,
    pub last_login: Option<OffsetDateTime>,
    pub failed_login_attempts: Option<i32>,
    pub account_locked: Option<bool>,
    pub password_changed_at: Option<OffsetDateTime>
}
use crate::db::pool;

impl User {
    pub async fn find_by_username(username: &str, pool: &sqlx::MySqlPool) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"SELECT 
                id, username, email, password_hash, role,
                created_at, updated_at, last_login,
                failed_login_attempts as `failed_login_attempts: i32`,
                account_locked as `account_locked: bool`,
                password_changed_at
            FROM users WHERE username = ?"#,
            username
        )
        .fetch_optional(pool)
        .await
    }
    pub async fn update_last_login(user_id: &str) -> Result<(), sqlx::Error> {
        let pool = pool::get_pool();
        sqlx::query!(
            "UPDATE users SET last_login = CURRENT_TIMESTAMP WHERE id = ?",
            user_id
        )
        .execute(pool)        
        .await?;
        Ok(())
    }

    pub async fn reset_failed_attempts(user_id: &str) -> Result<(), sqlx::Error> {
        let pool = pool::get_pool();
        sqlx::query!(
            "UPDATE users SET failed_login_attempts = 0 WHERE id = ?",
            user_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn increment_failed_attempts(user_id: &str) -> Result<(), sqlx::Error> {
        let pool = pool::get_pool();
        sqlx::query!(
            "UPDATE users SET failed_login_attempts = failed_login_attempts + 1 WHERE id = ?",
            user_id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}