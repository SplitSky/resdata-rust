use crate::api::models::Dataset; // data structures
use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

#[post("/insert")]
pub async fn insert(payload: web::Bytes) -> impl Responder {
    // Deserialize JSON manually using Serde
    let dataset: Dataset = match serde_json::from_slice(&payload) {
        Ok(data) => data,
        Err(_) => return HttpResponse::BadRequest().body("Invalid JSON"),
    };

    // Placeholder for database insertion logic
    println!("Inserting dataset with id: {}", dataset.id);

    // Manually serialize the dataset to JSON for the response
    match serde_json::to_string(&dataset) {
        Ok(json_data) => HttpResponse::Ok().body(json_data),
        Err(_) => HttpResponse::InternalServerError().body("Failed to serialize response"),
    }
}

#[get("/fetch/{id}")]
pub async fn fetch(id: web::Path<u32>) -> impl Responder {
    let dataset_id = id.into_inner();

    // Placeholder for database fetching logic
    println!("Fetching dataset with id: {}", dataset_id);

    // Create a dummy dataset
    let dummy_dataset = Dataset {
        id: dataset_id,
        name: "Sample Dataset".to_string(),
        data: "This is sample data".to_string(),
    };

    // Manually serialize the dataset to JSON for the response
    match serde_json::to_string(&dummy_dataset) {
        Ok(json_data) => HttpResponse::Ok().body(json_data),
        Err(_) => HttpResponse::InternalServerError().body("Failed to serialize response"),
    }
}

#[get("/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    // Placeholder for health check logic (e.g., ping database or services)
    // Here, we'll just return a simple JSON status

    // Manually create a JSON response using serde_json::json macro
    let health_status = json!({
        "status": "healthy",
        "database_connection": true,
        "other_service_status": "operational"
    });

    match serde_json::to_string(&health_status) {
        Ok(json_data) => HttpResponse::Ok().body(json_data),
        Err(_) => HttpResponse::InternalServerError().body("Failed to serialize health status"),
    }
}
