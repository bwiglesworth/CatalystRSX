use catalyst_rsx::db;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let pool = db::create_pool().await.expect("Failed to create pool");    db::init_db::initialize_database(&pool)
        .await
        .expect("Database initialization successful");
    println!("Database initialized with admin user");
}