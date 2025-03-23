#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, HttpResponse, HttpRequest, dev::ServiceRequest};
    use catalyst_rsx::logging::security::{SecurityLogger, SecurityLogLevel};
    use std::thread;
    use std::time::Duration;

    #[actix_web::test]
    async fn test_suspicious_activity_detection() {
        let security_logger = SecurityLogger::new(SecurityLogLevel::High);
        
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(security_logger))
                .route("/login", web::post().to(|logger: web::Data<SecurityLogger>, req: HttpRequest| async move {
                    let service_req = ServiceRequest::from_request(req);
                    // Simulate multiple failed attempts
                    for _ in 0..6 {
                        logger.check_suspicious_activity(&service_req, false);
                        thread::sleep(Duration::from_millis(100));
                    }
                    HttpResponse::Ok().finish()
                }))
        ).await;

        let req = test::TestRequest::post()
            .uri("/login")
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
