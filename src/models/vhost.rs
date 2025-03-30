use serde::{Serialize, Deserialize};
use time::OffsetDateTime;
use sqlx::MySqlPool;
//use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize, Deserialize, Clone)]
pub struct VirtualHost {
    pub id: String,
    pub domain: String,
    pub root_path: String,
    pub ssl_enabled: bool,
    pub ssl_cert_path: Option<String>,
    pub ssl_key_path: Option<String>,
    pub created_at: Option<OffsetDateTime>,
    pub updated_at: Option<OffsetDateTime>
}

impl VirtualHost {
    pub async fn create_table(pool: &MySqlPool) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"CREATE TABLE IF NOT EXISTS virtual_hosts (
                id VARCHAR(36) PRIMARY KEY,
                domain VARCHAR(255) NOT NULL UNIQUE,
                root_path VARCHAR(255) NOT NULL,
                ssl_enabled BOOLEAN DEFAULT false,
                ssl_cert_path VARCHAR(255),
                ssl_key_path VARCHAR(255),
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
            )"#
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}