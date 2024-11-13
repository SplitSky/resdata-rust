mod api;
mod model;
mod repository;

use actix_web::{middleware::Logger, web, App, HttpServer};
use api::{testing_insert::insert_dataset, testing_read::get_document}; // api functions
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
// use mongodb::{options::ClientOptions, Client};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    // fetch environmental variables
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let _pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Faield to create pool");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(web::Data::new(_pool.clone()))
            .service(insert_dataset)
            .service(get_document)
        // .service() // add more calls
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
