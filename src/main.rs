mod api;
mod model;
mod repository;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::{ftl::insert_dataset, ftl::get_document, authentication, permissions}; // api functions
use dotenv::dotenv;
use mongodb::{options::ClientOptions, Client};
use std::env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();
    // fetch environmental variables
    dotenv().ok();
    let _connection_string = env::var("MONGO_URI").expect("MONGO_URI must be set in the .env");

    println!("{}", _connection_string.to_string());
    // Parse and configure MongoDB client options
    let client_options = ClientOptions::parse(&_connection_string)
        .await
        .expect("Failed to Parse Mongo URI");
    let mongo_client =
        Client::with_options(client_options).expect("Failed to initialise MongoDB client");
    // wrap mongo client in Actix Data for shared state
    let _mongo_data = Data::new(mongo_client);

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            .app_data(_mongo_data.clone()) // MongoDB shared state
            .service(insert_dataset)
            .service(get_document)
        // .service() // add more calls
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
