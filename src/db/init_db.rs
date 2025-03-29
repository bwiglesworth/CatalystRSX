use sqlx::MySqlPool;
use crate::security::password::manager::PasswordManager;

pub async fn initialize_database(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    create_admin_user(pool).await
}

async fn create_admin_user(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let admin_email = std::env::var("ADMIN_EMAIL").expect("ADMIN_EMAIL must be set");
    let admin_password = std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set");
    
    let password_manager = PasswordManager::new(pool.clone());
    let hashed_password = password_manager.hash_password(&admin_password)
        .expect("Password hashing successful");

    sqlx::query!(
        r#"
        INSERT INTO users (username, email, password_hash, role)
        VALUES ('admin', ?, ?, 'admin')
        ON DUPLICATE KEY UPDATE password_hash = ?
        "#,
        admin_email,
        hashed_password,
        hashed_password
    )
    .execute(pool)
    .await?;

    Ok(())
}