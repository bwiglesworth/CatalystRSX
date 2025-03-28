#[cfg(test)]
mod tests {
    use actix_web::{test, App, web, HttpResponse};

    #[actix_web::test]
    async fn test_server_health() {
        let app = test::init_service(
            App::new()
                .route("/health", web::get().to(|| async { 
                    HttpResponse::Ok().body("Server is running") 
                }))
        ).await;

        let req = test::TestRequest::get()
            .uri("/health")
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        
        assert!(resp.status().is_success());
        
        let body = test::read_body(resp).await;
        assert_eq!(body, "Server is running");
    }

    #[actix_web::test]
async fn test_index_page() {
    let app = test::init_service(
        App::new()
            .route("/", web::get().to(index_handler))
    ).await;

    let req = test::TestRequest::get()
        .uri("/")
        .to_request();
    let resp = test::call_service(&app, req).await;
    
    assert!(resp.status().is_success());
    let body = test::read_body(resp).await;
    assert!(String::from_utf8_lossy(&body).contains("Enterprise-grade security"));
}
}
