use actix_web::{test, App, web, HttpResponse};
use catalyst_rsx::error::AppError;
use catalyst_rsx::middleware::error::error_handlers;

#[actix_web::test]
async fn test_validation_error() {
    let app = test::init_service(
        App::new()
            .wrap(error_handlers())
            .route("/", web::get().to(|| async { 
                Err::<HttpResponse, _>(AppError::ValidationError("test error".to_string()))
            }))
    ).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 400);
}

#[actix_web::test]
async fn test_auth_error() {
    let app = test::init_service(
        App::new()
            .wrap(error_handlers())
            .service(web::resource("/test").to(|| async { 
                Err::<HttpResponse, _>(AppError::AuthenticationError)
            }))
    ).await;

    let req = test::TestRequest::get().uri("/test").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 401);
}

#[actix_web::test]
async fn test_internal_error() {
    let app = test::init_service(
        App::new()
            .wrap(error_handlers())
            .service(web::resource("/test").to(|| async { 
                Err::<HttpResponse, _>(AppError::InternalError)
            }))
    ).await;

    let req = test::TestRequest::get().uri("/test").to_request();
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status().as_u16(), 500);
}