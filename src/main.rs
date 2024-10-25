use actix_web::{App, HttpServer};

// Import the api module
mod api;
use api::fetch::fetch;
use api::insert::insert;
use api::ping_test;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(insert) // Insert endpoint
            .service(fetch) // Fetch endpoint
                            // Endpoints to add: authenticate
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
