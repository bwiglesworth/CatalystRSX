use actix_web::{web, guard, HttpResponse};
use sqlx::MySqlPool;
pub async fn configure_vhosts(cfg: &mut web::ServiceConfig, pool: MySqlPool) {
    // Load virtual hosts from database
    let vhosts = sqlx::query!("SELECT domain, root_path FROM virtual_hosts")
        .fetch_all(&pool)
        .await
        .expect("Failed to load virtual hosts");

    // Configure each virtual host
    for vhost in vhosts {
        cfg.service(
            web::scope("/")
                .guard(guard::Host(vhost.domain.as_str()))
                .configure(move |app| {
                    app.service(
                        web::resource("/{path:.*}")
                            .route(web::get().to(move |path: web::Path<String>| {
                                let root = vhost.root_path.clone();
                                async move {
                                    // Handle the request based on vhost configuration
                                    HttpResponse::Ok().body(format!("Serving {} from {}", path, root))
                                }
                            }))
                    );
                })
        );
    }
}