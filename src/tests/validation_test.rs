use crate::validation::{UserInput, configure_validation};
use actix_web::{test, web, App, HttpResponse};
use validator::Validate;

#[actix_web::test]
async fn test_input_validation() {
    let app = test::init_service(
        App::new()
            .app_data(configure_validation())
            .route("/test", web::post().to(|input: web::Json<UserInput>| async move {
                match input.validate() {
                    Ok(_) => HttpResponse::Ok(),
                    Err(_) => HttpResponse::BadRequest(),
                }
            }))
    ).await;

    // Test valid input
    let valid_req = test::TestRequest::post()
        .uri("/test")
        .set_json(UserInput {
            name: "John Doe".to_string(),
            email: "john@example.com".to_string(),
        })
        .to_request();
    
    let resp = test::call_service(&app, valid_req).await;
    assert_eq!(resp.status(), 200);

    // Test invalid input
    let invalid_req = test::TestRequest::post()
        .uri("/test")
        .set_json(UserInput {
            name: "J".to_string(), // too short
            email: "not-an-email".to_string(),
        })
        .to_request();
    
    let resp = test::call_service(&app, invalid_req).await;
    assert_eq!(resp.status(), 400);
}
