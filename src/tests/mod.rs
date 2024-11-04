#[cfg(test)]
mod tests {
    use crate::api::{fetch_dataset, health_check, insert_dataset};
    use crate::repository::MongoRepo;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_health_check() {
        let repo = MongoRepo::init().await;
        let app = test::init_service(
            App::new()
                .app_data(web::Data::new(repo.clone()))
                .route("/health", web::get().to(health_check)),
        )
        .await;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
