use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use log::info;
use middleware::db::Db;
use serde::Deserialize;
use std::env;
mod handlers;
mod middleware;
mod model;

#[derive(Deserialize)]
struct Config {
    mongo_uri: String,
    database_name: String,
    collection_name: String,
}

impl Config {
    fn from_env() -> Self {
        dotenv().ok();
        Self {
            mongo_uri: env::var("MONGO_URI").expect("MONGO_URI must be set"),
            database_name: env::var("DATABASE_NAME").expect("DATABASE_NAME must be set"),
            collection_name: env::var("COLLECTION_NAME").expect("COLLECTION_NAME must be set"),
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let config = Config::from_env();

    let db = Db::new(
        &config.mongo_uri,
        &config.database_name,
        &config.collection_name,
    )
    .await
    .expect("Failed to initialize database");

    info!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route(
                "/insert",
                web::post().to(handlers::handlers::insert_dataset),
            )
            .service(
                web::resource("/get/{id}").route(web::get().to(handlers::handlers::get_dataset)),
            )
            .service(web::resource("/list").route(web::get().to(handlers::handlers::list_datasets)))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
