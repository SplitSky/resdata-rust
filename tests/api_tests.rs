use actix_web::{test, App};
use resdata_rust::api::ftl::{insert_dataset, get_document};
use resdata_rust::model::MyDocument;

#[actix_rt::test]
async fn test_insert_dataset() {
    let mut app = test::init_service(App::new().service(insert_dataset)).await;
    let req = test::TestRequest::post()
        .uri("/insert")
        .set_json(&MyDocument {
            id: None,
            name: "Test".to_string(),
            description: "Test description".to_string(),
        })
        .to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_get_document() {
    let mut app = test::init_service(App::new().service(get_document)).await;
    let req = test::TestRequest::get().uri("/get/1").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}