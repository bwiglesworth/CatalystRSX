#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, HttpResponse, HttpRequest, dev::ServiceRequest};
    use catalyst_rsx::logging::security::{SecurityLogger, SecurityLogLevel};

    #[actix_web::test]
    async fn test_auth_tracking() {
        let security_logger = SecurityLogger::new(SecurityLogLevel::High);
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(security_logger))
                .route("/login", web::post().to(|logger: web::Data<SecurityLogger>, req: HttpRequest| async move {
                    let service_req = ServiceRequest::from_request(req);
                    logger.track_auth_attempt(&service_req, true);
                    HttpResponse::Ok().finish()
                }))
        ).await;

        // Test with custom User-Agent
        let req = test::TestRequest::post()
            .uri("/login")
            .insert_header(("User-Agent", "test-client"))
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
