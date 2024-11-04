use crate::api::{fetch_dataset, health_check, insert_dataset};
use crate::repository::MongoRepo;
use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod api;
mod models;
mod repository;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mongo_repo = MongoRepo::init().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(mongo_repo.clone()))
            .route("/health", web::get().to(health_check))
            .route("/dataset", web::post().to(insert_dataset))
            .route("/dataset/{id}", web::get().to(fetch_dataset))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
