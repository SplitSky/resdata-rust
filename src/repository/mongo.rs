use actix_web::{web, HttpResponse, Responder};

use mongodb::Database;
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    db: Arc<Database>, // Use an Arc to safely share the database across threads
}

// Example handler that lists collections
async fn list_collections(data: web::Data<AppState>) -> impl Responder {
    let db = &data.db; // Get the db client reference
                       // List collections in the specified database
    match db.list_collection_names(None).await {
        Ok(collections) => HttpResponse::Ok().json(collections),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {}", err)),
    }
}

// Simple home handler
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the MongoDB + Actix Web integration!")
}
