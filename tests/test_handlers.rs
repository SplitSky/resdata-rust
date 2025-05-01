use actix_web::{test, web, App};
use resdata_rust::handlers::handlers::{insert_dataset, list_datasets};
use resdata_rust::middleware::db::Db;
use resdata_rust::model::data_structs::Dataset;

#[actix_web::test]
async fn test_insert_and_list_datasets() {
    let db = Db::new("mongodb://localhost:27017", "test_db", "test_collection")
        .await
        .unwrap();

    let app = test::init_service(
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/insert", web::post().to(insert_dataset))
            .route("/list", web::get().to(list_datasets)),
    )
    .await;

    let dataset = Dataset {
        id: None,
        data: vec![100, 200, 300],
        headers: vec!["Header1".to_string()],
        author: vec!["Author1".to_string()],
    };

    let req = test::TestRequest::post()
        .uri("/insert")
        .set_json(&dataset)
        .to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let req = test::TestRequest::get().uri("/list").to_request();
    let resp: Vec<Dataset> = test::call_and_read_body_json(&app, req).await;
    assert!(!resp.is_empty());
    assert_eq!(resp[0].data, dataset.data);
}
