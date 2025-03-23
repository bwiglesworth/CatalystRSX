#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, HttpResponse};
    use actix_session::Session;
    

    #[actix_web::test]
    async fn test_session_timeout() {
        let app = test::init_service(
            App::new()
                .wrap(crate::auth::session::configure_session())
                .route("/set", web::get().to(|session: Session| async move {
                    session.insert("test_key", "test_value").unwrap();
                    HttpResponse::Ok().finish()
                }))
                .route("/get", web::get().to(|session: Session| async move {
                    match session.get::<String>("test_key").unwrap() {
                        Some(value) => HttpResponse::Ok().body(value),
                        None => HttpResponse::NotFound().finish()
                    }
                }))
        ).await;

        // Set session data
        let req = test::TestRequest::get().uri("/set").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());

        // Get session cookie
        let cookie = resp.response().cookies().next().unwrap();
        
        // Verify session data is accessible
        let req = test::TestRequest::get()
            .uri("/get")
            .cookie(cookie.clone())
            .to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_web::test]
    async fn test_session_security() {
        let app = test::init_service(
            App::new()
                .wrap(crate::auth::session::configure_session())
                .route("/session", web::get().to(|session: Session| async move {
                    session.insert("secure_key", "secure_value").unwrap();
                    HttpResponse::Ok().finish()
                }))
        ).await;

        let req = test::TestRequest::get().uri("/session").to_request();
        let resp = test::call_service(&app, req).await;
        
        // Verify secure cookie attributes
        let cookie = resp.response().cookies().next().unwrap();
        assert!(cookie.secure().unwrap_or(false));
        assert!(cookie.http_only().unwrap_or(false));
    }
}
