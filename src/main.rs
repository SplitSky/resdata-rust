use actix_web::{App, HttpServer};

// Import the api module
mod api;
use api::ftl;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(ftl::insert)
            .service(ftl::fetch) // Fetch endpoint
            .service(ftl::healthcheck)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
