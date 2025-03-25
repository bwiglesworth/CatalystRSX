#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use catalyst_rsx::middleware::SecurityHeaders;    
    
    #[actix_web::test]
    async fn test_security_headers() {
        let app = test::init_service(
            actix_web::App::new()
                .wrap(
                    actix_web::middleware::DefaultHeaders::new()
                        .add(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
                        .add(("X-Frame-Options", "DENY"))
                        .add(("X-Content-Type-Options", "nosniff"))
                        .add(("Content-Security-Policy", "default-src 'self'"))
                )
                .route("/", actix_web::web::get().to(|| async { Ok::<_, actix_web::Error>(()) }))
        ).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;

        assert!(resp.headers().contains_key("strict-transport-security"));
        assert!(resp.headers().contains_key("x-frame-options"));
        assert!(resp.headers().contains_key("x-content-type-options"));
        assert!(resp.headers().contains_key("content-security-policy"));
    }

    #[actix_web::test]
    async fn test_csp_headers() {
        let app = test::init_service(
            App::new()
                .wrap(SecurityHeaders)
        ).await;

        let req = test::TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
        
        let csp = resp.headers().get("Content-Security-Policy").unwrap();
        assert!(csp.to_str().unwrap().contains("default-src 'self'"));
        assert!(csp.to_str().unwrap().contains("script-src 'self'"));
        assert!(csp.to_str().unwrap().contains("style-src 'self'"));
        assert!(csp.to_str().unwrap().contains("img-src 'self'"));
    }
}