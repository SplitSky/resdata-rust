// src/tests/integration_tests.rs
use actix_web::http::StatusCode;
use actix_web::{test, App};
use resdataRust::api::fetch::fetch;
use resdataRust::api::insert::{insert, Dataset};

#[actix_web::test]
async fn test_insert() {
    let app = test::init_service(App::new().service(insert)).await;

    let payload = Dataset {
        id: 1,
        name: String::from("My Dataset"),
        data: String::from("Sample data content"),
    };

    let req = test::TestRequest::post()
        .uri("/insert")
        .set_json(&payload)
        .to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let result: Dataset = test::read_body_json(resp).await;
    assert_eq!(result, payload);
}

#[actix_web::test]
async fn test_fetch() {
    let app = test::init_service(App::new().service(fetch)).await;

    let req = test::TestRequest::get().uri("/fetch/1").to_request();

    let resp = test::call_service(&app, req).await;

    assert_eq!(resp.status(), StatusCode::OK);

    let result: Dataset = test::read_body_json(resp).await;
    let expected = Dataset {
        id: 1,
        name: "Sample Dataset".to_string(),
        data: "This is sample data".to_string(),
    };
    assert_eq!(result, expected);
}
