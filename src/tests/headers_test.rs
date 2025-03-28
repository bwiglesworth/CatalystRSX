#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, HttpResponse};
    use actix_web::test::TestRequest;
    use crate::middleware::SecurityHeaders;
    use crate::security::headers::{
        ReferrerPolicy,
        ReferrerPolicyBuilder,
        FeaturePolicy,
        FeaturePolicyBuilder,
        ExpectCTBuilder,
        XFrameOptions,
        XFrameOptionsBuilder
    };
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

    #[actix_web::test]
    async fn test_referrer_policy() {
        let builder = ReferrerPolicyBuilder::new();
        let policy = builder
            .set_policy(ReferrerPolicy::StrictOrigin)
            .build();
        
        assert_eq!(policy, "strict-origin");
    }

    #[actix_web::test]
    async fn test_security_headers_feature_policy() {
        let app = test::init_service(
            App::new()
                .wrap(SecurityHeaders::new())
                .route("/", web::get().to(|| async { HttpResponse::Ok().body("test") }))
        ).await;

        let req = TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
            
        let feature_policy = resp.headers().get("Feature-Policy").unwrap();
        assert!(feature_policy.to_str().unwrap().contains("camera 'none'"));
        assert!(feature_policy.to_str().unwrap().contains("microphone 'self'"));
        assert!(feature_policy.to_str().unwrap().contains("payment https://payment.example.com"));
    }

    #[actix_web::test]
    async fn test_feature_policy_empty_builder() {
        let builder = FeaturePolicyBuilder::new();
        assert_eq!(builder.build(), "");
    }

    #[actix_web::test]
    async fn test_feature_policy_multiple_features() {
        let mut builder = FeaturePolicyBuilder::new();
        builder
            .add_feature("geolocation", FeaturePolicy::Self_)
            .add_feature("camera", FeaturePolicy::None)
            .add_feature("autoplay", FeaturePolicy::All);
        
        let policy = builder.build();
        assert!(policy.contains("geolocation 'self'"));
        assert!(policy.contains("camera 'none'"));
        assert!(policy.contains("autoplay *"));
    }

    #[actix_web::test]
    async fn test_feature_policy_origins_list() {
        let mut builder = FeaturePolicyBuilder::new();
        let origins = vec![
            "https://trusted1.example.com".to_string(),
            "https://trusted2.example.com".to_string()
        ];
        builder.add_feature("payment", FeaturePolicy::Origins(origins));
        
        let policy = builder.build();
        assert!(policy.contains("payment https://trusted1.example.com https://trusted2.example.com"));
    }

    #[actix_web::test]
    async fn test_expect_ct_default() {
        let builder = ExpectCTBuilder::new();
        let header = builder.build();
        assert!(header.contains("max-age=86400"));
        assert!(header.contains("enforce"));
    }

    #[actix_web::test]
    async fn test_expect_ct_custom() {
        let builder = ExpectCTBuilder::new()
            .max_age(3600)
            .enforce(false)
            .report_uri("https://example.com/report".to_string());
        
        let header = builder.build();
        assert!(header.contains("max-age=3600"));
        assert!(!header.contains("enforce"));
        assert!(header.contains("report-uri=\"https://example.com/report\""));
    }

    #[actix_web::test]
    async fn test_x_frame_options_default() {
        let builder = XFrameOptionsBuilder::new();
        let header = builder.build();
        assert_eq!(header, "SAMEORIGIN");
    }

    #[actix_web::test]
    async fn test_x_frame_options_deny() {
        let builder = XFrameOptionsBuilder::new()
            .set_option(XFrameOptions::Deny);
        let header = builder.build();
        assert_eq!(header, "DENY");
    }

    #[actix_web::test]
    async fn test_x_frame_options_allow_from() {
        let builder = XFrameOptionsBuilder::new()
            .set_option(XFrameOptions::AllowFrom("https://trusted.example.com".to_string()));
        let header = builder.build();
        assert_eq!(header, "ALLOW-FROM https://trusted.example.com");
    }

    #[actix_web::test]
    async fn test_x_frame_options_middleware() {
        let app = test::init_service(
            App::new()
                .wrap(SecurityHeaders::new())
                .route("/", web::get().to(|| async { HttpResponse::Ok().body("test") }))
        ).await;

        let req = TestRequest::get().uri("/").to_request();
        let resp = test::call_service(&app, req).await;
    
        let x_frame_options = resp.headers().get("X-Frame-Options").unwrap();
        assert_eq!(x_frame_options.to_str().unwrap(), "SAMEORIGIN");
    }
}