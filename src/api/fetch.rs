// src/api/fetch.rs
use actix_web::{get, web, HttpResponse, Responder};
use serde::Serialize;

#[derive(Serialize)]
pub struct Dataset {
    pub id: u32,
    pub name: String,
    pub data: String,
}

#[get("/fetch/{id}")]
pub async fn fetch(id: web::Path<u32>) -> impl Responder {
    let dataset_id = id.into_inner();

    // Placeholder for database fetching logic
    println!("Fetching dataset with id: {}", dataset_id);

    let dummy_dataset = Dataset {
        id: dataset_id,
        name: "Sample Dataset".to_string(),
        data: "This is sample data".to_string(),
    };

    HttpResponse::Ok().json(dummy_dataset) // Responds with the fetched dataset
}
