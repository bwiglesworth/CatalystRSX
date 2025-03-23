use actix_web::{test, App, web, HttpResponse};
use crate::error::AppError;
use crate::middleware::error::error_handlers;

#[cfg(test)]
mod tests {
    use actix_web::{test, App, web, HttpResponse};
    use crate::error::AppError;
    use crate::middleware::error::error_handlers;

    #[actix_web::test]
    async fn test_auth_error() {
        let app = test::init_service(
            App::new()
                .configure(error_handlers)
                .route("/auth", web::get().to(|| async { 
                    Err::<HttpResponse, _>(AppError::AuthenticationError)
                }))
        ).await;

        let req = test::TestRequest::get().uri("/auth").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 401);
    }

    #[actix_web::test]
    async fn test_internal_error() {
        let app = test::init_service(
            App::new()
                .configure(error_handlers)
                .route("/error", web::get().to(|| async { 
                    Err::<HttpResponse, _>(AppError::InternalError)
                }))
        ).await;

        let req = test::TestRequest::get().uri("/error").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_validation_error() {
        let app = test::init_service(
            App::new()
                .configure(error_handlers)
                .route("/validate", web::get().to(|| async { 
                    Err::<HttpResponse, _>(AppError::ValidationError("Invalid input".to_string()))
                }))
        ).await;

        let req = test::TestRequest::get().uri("/validate").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 400);
    }}