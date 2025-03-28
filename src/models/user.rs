use sqlx::types::time::OffsetDateTime;
#[derive(sqlx::FromRow, Debug)]
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
    pub account_locked: Option<i32>,
    pub password_changed_at: Option<OffsetDateTime>
}