use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use mongodb::{Client, options::ClientOptions, Database};
use std::sync::Arc;
use tokio;

#[derive(Clone)]
struct AppState {
    db: Arc<Database>,  // Use an Arc to safely share the database across threads
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // MongoDB connection details
    let username = "your_username";
    let password = "your_password";
    let host = "localhost";
    let port = 27017;
    let auth_db = "admin";  // Database where the user is defined

    // Build the MongoDB URI
    let uri = format!(
        "mongodb://{}:{}@{}:{}/?authSource={}",
        username, password, host, port, auth_db
    );

    // Parse MongoDB client options
    let client_options = ClientOptions::parse(&uri).await.expect("Failed to parse URI");
    let client = Client::with_options(client_options).expect("Failed to initialize MongoDB client");

    // Select a database
    let database = client.database("your_database");
    
    // Wrap the database in Arc to safely share it across Actix threads
    let db = Arc::new(database);

    // Define AppState with MongoDB client
    let app_state = AppState { db };

    // Start the Actix web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(app_state.clone())) // Share AppState in the app
            .route("/", web::get().to(home))
            .route("/collections", web::get().to(list_collections))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// Example handler that lists collections
async fn list_collections(data: web::Data<AppState>) -> impl Responder {
    let db = &data.db;

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
