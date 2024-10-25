// src/api/insert.rs
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;
use serde::Serialize;

#[derive(Serialize, Deserialize)]
pub struct Dataset {
    pub id: u32,
    pub name: String,
    pub data: String,
}

#[post("/insert")]
pub async fn insert(dataset: web::Json<Dataset>) -> impl Responder {
    // Placeholder for database insertion logic
    println!("Inserting dataset with id: {}", dataset.id);

    HttpResponse::Ok().json(dataset.0) // Responds with the inserted dataset
}
