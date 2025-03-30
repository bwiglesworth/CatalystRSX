use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use sqlx::Error;
use uuid::Uuid;
use catalyst_rsx::security::password::manager::PasswordManager;


async fn connect_as_root() -> Result<MySqlPool, Error> {
    let root_user = std::env::var("MARIADB_ROOT_USER").expect("MARIADB_ROOT_USER must be set");
    let root_password = std::env::var("MARIADB_ROOT_PASSWORD").expect("MARIADB_ROOT_PASSWORD must be set");
    
    let root_url = format!("mysql://{}:{}@localhost", root_user, root_password);
    MySqlPoolOptions::new()
        .max_connections(1)
        .connect(&root_url)
        .await
}

async fn connect_as_app_user() -> Result<MySqlPool, Error> {
    let db_user = std::env::var("DB_USER").expect("DB_USER must be set");
    let db_password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    
    let app_url = format!("mysql://{}:{}@localhost/catalystrsx", db_user, db_password);
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&app_url)
        .await
}

async fn create_database(pool: &MySqlPool) -> Result<(), Error> {
    // Check if database exists
    let db_exists = sqlx::query("SELECT SCHEMA_NAME FROM INFORMATION_SCHEMA.SCHEMATA WHERE SCHEMA_NAME = 'catalystrsx'")
        .fetch_optional(pool)
        .await?;

    if db_exists.is_some() {
        println!("Database 'catalystrsx' already exists");
    } else {
        sqlx::query("CREATE DATABASE catalystrsx")
            .execute(pool)
            .await?;
        println!("Database 'catalystrsx' created successfully");
    }
    Ok(())
}


async fn check_user_exists(pool: &MySqlPool, username: &str) -> Result<bool, Error> {
    let result = sqlx::query("SELECT 1 FROM mysql.user WHERE user = ?")
        .bind(username)
        .fetch_optional(pool)
        .await?;
    Ok(result.is_some())
}

async fn create_db_user(pool: &MySqlPool) -> Result<(), Error> {
    let username = std::env::var("DB_USER").expect("DB_USER must be set");
    let password = std::env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    
    let query = format!(
        "CREATE USER '{}'@'localhost' IDENTIFIED BY '{}' REQUIRE SSL",
        username, password
    );
    
    sqlx::query(&query)
        .execute(pool)
        .await?;
        
    println!("Database user '{}' created with SSL requirement", username);
    Ok(())
}

async fn grant_privileges(pool: &MySqlPool) -> Result<(), Error> {
    let username = std::env::var("DB_USER").expect("DB_USER must be set");
    
    let query = format!(
        "GRANT ALL PRIVILEGES ON catalystrsx.* TO '{}'@'localhost'",
        username
    );
    
    sqlx::query(&query)
        .execute(pool)
        .await?;
    
    sqlx::query("FLUSH PRIVILEGES")
        .execute(pool)
        .await?;
    
    println!("Privileges granted to user '{}'", username);
    Ok(())
}

async fn create_users_table(pool: &MySqlPool) -> Result<(), Error> {
    let query = r#"
        CREATE TABLE IF NOT EXISTS users (
            id VARCHAR(36) PRIMARY KEY,
            username VARCHAR(255) NOT NULL UNIQUE,
            email VARCHAR(255) NOT NULL UNIQUE,
            password_hash VARCHAR(255) NOT NULL,
            role VARCHAR(50) NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
            last_login TIMESTAMP NULL,
            failed_login_attempts INT DEFAULT 0,
            account_locked BOOLEAN DEFAULT FALSE,
            password_changed_at TIMESTAMP NULL
        )
    "#;

    let table_exists = sqlx::query("SHOW TABLES LIKE 'users'")
        .fetch_optional(pool)
        .await?;

    if table_exists.is_some() {
        println!("Table 'users' already exists");
    } else {
        sqlx::query(query)
            .execute(pool)
            .await?;
        println!("Table 'users' created successfully");
    }
    Ok(())
}

async fn create_virtual_hosts_table(pool: &MySqlPool) -> Result<(), Error> {
    let query = r#"
        CREATE TABLE IF NOT EXISTS virtual_hosts (
            id VARCHAR(36) PRIMARY KEY,
            domain VARCHAR(255) NOT NULL UNIQUE,
            root_path VARCHAR(255) NOT NULL,
            ssl_enabled BOOLEAN DEFAULT false,
            ssl_cert_path VARCHAR(255),
            ssl_key_path VARCHAR(255),
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
        )
    "#;

    let table_exists = sqlx::query("SHOW TABLES LIKE 'virtual_hosts'")
        .fetch_optional(pool)
        .await?;

    if table_exists.is_some() {
        println!("Table 'virtual_hosts' already exists");
    } else {
        sqlx::query(query)
            .execute(pool)
            .await?;
        println!("Table 'virtual_hosts' created successfully");
    }
    Ok(())
}

async fn create_admin_user(pool: &MySqlPool) -> Result<(), Error> {
    let admin_email = std::env::var("ADMIN_EMAIL").expect("ADMIN_EMAIL must be set");
    let admin_password = std::env::var("ADMIN_PASSWORD").expect("ADMIN_PASSWORD must be set");
    
    let user_exists = sqlx::query!(
        "SELECT * FROM users WHERE email = ?",
        admin_email
    )
    .fetch_optional(pool)
    .await?;

    if user_exists.is_some() {
        println!("Admin user already exists");
        return Ok(());
    }

    let password_manager = PasswordManager::new(pool.clone());
    let password_hash = password_manager.hash_password(&admin_password)
        .expect("Failed to hash password");

    sqlx::query!(
        r#"
        INSERT INTO users (id, username, email, password_hash, role, created_at, updated_at)
        VALUES (?, ?, ?, ?, ?, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
        "#,
        Uuid::new_v4().to_string(),
        "admin",
        admin_email,
        password_hash,
        "admin"
    )
    .execute(pool)
    .await?;

    println!("Admin user created successfully");
    Ok(())
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    
    let root_pool = connect_as_root()
        .await
        .expect("Failed to connect as root");

    create_database(&root_pool)
        .await
        .expect("Failed to create database");

    let db_user = std::env::var("DB_USER").expect("DB_USER must be set");

    let user_exists = check_user_exists(&root_pool, &db_user)
        .await
        .expect("Failed to check user existence");

    if !user_exists {
        create_db_user(&root_pool)
            .await
            .expect("Failed to create database user");
            
        grant_privileges(&root_pool)
            .await
            .expect("Failed to grant privileges");
    }

    // Connect as application user to create tables
    let app_pool = connect_as_app_user()
    .await
    .expect("Failed to connect as application user");

    create_users_table(&app_pool)
        .await
        .expect("Failed to create users table");
   
    create_virtual_hosts_table(&app_pool)
    .await
    .expect("Failed to create virtual hosts table");

    // THIS STARTS ENTRY OF DATABASE ELEMENTS **** ONLY PLACE STUFF TO BE ADDED TO THE DATABASE BELOW THIS LINE
    create_admin_user(&app_pool)
    .await
    .expect("Failed to create admin user");


}