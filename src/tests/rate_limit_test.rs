use actix_web::{test, App, HttpResponse, web};
use actix_governor::{Governor, GovernorConfigBuilder};
use std::net::SocketAddr;

#[actix_web::test]
async fn test_rate_limiting() {
    let governor_conf = GovernorConfigBuilder::default()
        .per_second(2)
        .burst_size(1)
        .finish()
        .unwrap();

    let app = test::init_service(
        App::new()
            .wrap(Governor::new(&governor_conf))
            .route("/", web::get().to(|| HttpResponse::Ok()))
    ).await;

    // Create test socket address
    let test_addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    
    // First request should succeed
    let req = test::TestRequest::get()
        .uri("/")
        .peer_addr(test_addr)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 200);

    // Second request within the same second should be rate limited
    let req = test::TestRequest::get()
        .uri("/")
        .peer_addr(test_addr)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 429);
}