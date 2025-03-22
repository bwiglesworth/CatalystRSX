#[cfg(test)]
mod tests {
    use actix_web::test;
    use catalyst_rsx::validation::UserInput;

    #[actix_web::test]
    async fn test_validation() {
        let app = test::init_service(
            actix_web::App::new()
                .configure(catalyst_rsx::validation::configure_validation)
        ).await;

        let req = test::TestRequest::post()
            .uri("/validate")
            .set_json(UserInput {
                username: "test".to_string(),
                email: "test@example.com".to_string(),
            })
            .to_request();

        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}