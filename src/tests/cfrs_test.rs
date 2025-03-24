#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, HttpResponse};
    use actix_session::{Session, SessionMiddleware, storage::CookieSessionStore};
    use actix_web::cookie::Key;
    use crate::middleware::csrf::CsrfMiddleware;

    #[actix_web::test]
    async fn test_csrf_protection() {
        let app = test::init_service(
            App::new()
                .wrap(SessionMiddleware::new(
                    CookieSessionStore::default(),
                    Key::generate()
                ))
                .wrap(CsrfMiddleware)
                .route("/test", web::post().to(|| async { HttpResponse::Ok().finish() }))
        ).await;

        // Test GET request (should pass and set token)
        let req = test::TestRequest::get().uri("/test").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Test POST without token (should fail)
        let req = test::TestRequest::post().uri("/test").to_request();
        let resp = test::call_service(&app, req).await;
        assert_eq!(resp.status().as_u16(), 403);
    }
}
