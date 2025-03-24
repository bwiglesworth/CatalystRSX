#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, HttpResponse, Error};
    use catalyst_rsx::middleware::error::ErrorHandler;
    use std::io::{Error as IoError, ErrorKind};

    fn create_test_error(msg: &str) -> Error {
        Error::from(IoError::new(ErrorKind::Other, msg))
    }

    #[actix_web::test]
    async fn test_validation_error() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("validation error"))
                }))
        ).await;

        let req = test::TestRequest::post().uri("/validate").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_authentication_error() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("authentication error"))
                }))
        ).await;

        let req = test::TestRequest::get().uri("/auth").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_authorization_error() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("authorization error"))
                }))
        ).await;

        let req = test::TestRequest::get().uri("/admin").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_not_found_error() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("resource not found"))
                }))
        ).await;

        let req = test::TestRequest::get().uri("/missing").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_database_error() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("database connection failed"))
                }))
        ).await;

        let req = test::TestRequest::get().uri("/users").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_rate_limit_error() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("rate limit exceeded"))
                }))
        ).await;

        let req = test::TestRequest::get().uri("/api").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_session_error() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("session expired"))
                }))
        ).await;

        let req = test::TestRequest::get().uri("/session").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_payload_error() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("payload too large"))
                }))
        ).await;

        let req = test::TestRequest::post().uri("/upload").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_media_type_error() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("unsupported media type"))
                }))
        ).await;

        let req = test::TestRequest::post().uri("/media").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_timeout_error() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("request timeout"))
                }))
        ).await;

        let req = test::TestRequest::get().uri("/slow").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_configuration_error() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("invalid configuration"))
                }))
        ).await;

        let req = test::TestRequest::get().uri("/config").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_middleware_error() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("middleware failure"))
                }))
        ).await;

        let req = test::TestRequest::get().uri("/middleware").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_template_error() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("template rendering failed"))
                }))
        ).await;

        let req = test::TestRequest::get().uri("/template").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_serialization_error() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("serialization failed"))
                }))
        ).await;

        let req = test::TestRequest::get().uri("/json").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_external_service_error() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("external service unavailable"))
                }))
        ).await;

        let req = test::TestRequest::get().uri("/external").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_http_method_errors() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("method error"))
                }))
        ).await;

        // Test PUT error
        let put_req = test::TestRequest::put()
            .uri("/api/resource")
            .to_request();
        let resp = test::call_service(&app, put_req).await;
        assert_eq!(resp.status(), 500);

        // Test DELETE error
        let delete_req = test::TestRequest::delete()
            .uri("/api/resource/1")
            .to_request();
        let resp = test::call_service(&app, delete_req).await;
        assert_eq!(resp.status(), 500);

        // Test PATCH error
        let patch_req = test::TestRequest::patch()
            .uri("/api/resource/1")
            .to_request();
        let resp = test::call_service(&app, patch_req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_query_param_errors() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("invalid query parameters"))
                }))
        ).await;

        let req = test::TestRequest::get()
            .uri("/api/search?invalid=true&malformed=yes")
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_request_body_size_errors() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("body size exceeded"))
                }))
        ).await;

        let req = test::TestRequest::post()
            .uri("/api/data")
            .insert_header(("content-length", "1073741824"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }

    #[actix_web::test]
    async fn test_malformed_body_errors() {
        let app = test::init_service(
            App::new()
                .wrap(ErrorHandler)
                .default_service(web::to(|| async { 
                    Err::<HttpResponse, Error>(create_test_error("malformed request body"))
                }))
        ).await;

        let req = test::TestRequest::post()
            .uri("/api/data")
            .set_payload("invalid{json:data")
            .insert_header(("content-type", "application/json"))
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status(), 500);
    }
}   