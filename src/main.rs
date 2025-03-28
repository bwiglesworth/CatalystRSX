use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use actix_governor::{Governor, GovernorConfigBuilder};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use catalyst_rsx::auth::admin::{AdminGuard, admin_login};
use catalyst_rsx::auth::session::configure_session;
use catalyst_rsx::routing::handlers::{dashboard_handler, admin_login_page, index_handler};
use catalyst_rsx::config::Config;
use catalyst_rsx::db::pool::init_pool;
use sqlx::mysql::MySqlPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"))
        .await
        .expect("Failed to create pool");

    init_pool(pool);
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(5)
        .finish()
        .unwrap();

    let config = Config::from_env()
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls())?;
    builder.set_certificate_file(&config.server.tls_cert_path, SslFiletype::PEM)?;
    builder.set_private_key_file(&config.server.tls_key_path, SslFiletype::PEM)?;

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(Governor::new(&governor_conf))
            .wrap(configure_session())
            .route("/", web::get().to(index_handler))
            .service(
                web::scope("/admin")
                    // Public admin routes
                    .route("/login", web::get().to(admin_login_page))
                    .route("/login", web::post().to(admin_login))
                    // Protected admin routes
                    .service(
                        web::scope("/dashboard")
                            .wrap(AdminGuard::new())
                            .route("", web::get().to(dashboard_handler))
                    )
            )    })
    .bind_openssl(&format!("{}:{}", config.server.host, config.server.port), builder)?
    .run()
    .await
}