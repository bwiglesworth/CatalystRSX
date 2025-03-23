#[cfg(test)]
mod tests {
    use actix_web::{test, web, App, HttpResponse};
    use crate::auth::guard::SessionGuard;

    #[actix_web::test]
    async fn test_protected_route_unauthorized() {
        let app = test::init_service(
            App::new()
                .service(
                    web::scope("/api")
                        .wrap(SessionGuard)
                        .route("/test", web::get().to(|| async { 
                            HttpResponse::Ok().finish() 
                        }))
                )
        ).await;

        let req = test::TestRequest::get().uri("/api/test").to_request();
        let resp = test::try_call_service(&app, req).await;
        assert!(resp.is_err());
    }

    #[actix_web::test]
    async fn test_protected_route_authorized() {
        let app = test::init_service(
            App::new()
                .wrap(actix_session::SessionMiddleware::new(
                    actix_session::storage::CookieSessionStore::default(),
                    actix_web::cookie::Key::from(&[0; 64])
                ))
                .service(
                    web::scope("/api")
                        .route("/login", web::post().to(|session: actix_session::Session| async move {
                            session.insert("user_id", "test_user").unwrap();
                            HttpResponse::Ok().finish()
                        }))
                        .service(
                            web::scope("")
                                .wrap(SessionGuard)
                                .route("/test", web::get().to(|| async {
                                    HttpResponse::Ok().finish()
                                }))
                        )
                )
        ).await;

        let login_req = test::TestRequest::post()
            .uri("/api/login")
            .to_request();
        let login_resp = test::call_service(&app, login_req).await;
        
        let session_cookie = login_resp.response().cookies().next().unwrap();
        
        let req = test::TestRequest::get()
            .uri("/api/test")
            .cookie(session_cookie)
            .to_request();
            
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}