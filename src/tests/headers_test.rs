use actix_web::{test, App, HttpResponse, web};
use actix_web::middleware::DefaultHeaders;

#[actix_web::test]
async fn test_security_headers() {
    let app = test::init_service(
        App::new()
            .wrap(
                DefaultHeaders::new()
                    .add(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
                    .add(("X-Frame-Options", "DENY"))
                    .add(("X-Content-Type-Options", "nosniff"))
                    .add(("Content-Security-Policy", "default-src 'self'"))
                    .add(("X-XSS-Protection", "1; mode=block"))
                    .add(("Referrer-Policy", "strict-origin-when-cross-origin"))
            )
            .route("/", web::get().to(|| HttpResponse::Ok()))
    ).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;

    assert!(resp.headers().contains_key("strict-transport-security"));
    assert!(resp.headers().contains_key("x-frame-options"));
    assert!(resp.headers().contains_key("x-content-type-options"));
    assert!(resp.headers().contains_key("content-security-policy"));
    assert!(resp.headers().contains_key("x-xss-protection"));
    assert!(resp.headers().contains_key("referrer-policy"));
}
