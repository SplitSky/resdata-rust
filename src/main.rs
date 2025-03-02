use actix_web::{App, HttpServer, Responder, web};
use dotenv::dotenv;
use log::info;
use middleware::db::Db;
use mongodb::Client;
use serde::{Deserialize, Serialize};
use std::env;
mod handlers;
mod middleware;
mod model;

#[derive(Serialize, Deserialize, Clone)]
struct MyData {
    id: i32,
    value: String,
}

async fn insert_data(client: web::Data<Client>) -> impl Responder {
    let db = client.database(&env::var("DATABASE_NAME").unwrap());
    let collection = db.collection::<MyData>("my_collection");
    let data = MyData {
        id: 1,
        value: "Random Value".to_string(),
    };
    collection.insert_one(data.clone(), None).await.unwrap();
    web::Json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI must be set");
    // let client_options = ClientOptions::parse(mongo_uri).await.unwrap();
    // let client = Client::with_options(client_options).unwrap();
    let db = Db::new(&mongo_uri).await;
    info!("Starting server at http://127.0.0.1:8080");
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            .route("/insert", web::get().to(insert_data))
            .service(
                web::resource("/get/{id}").route(web::get().to(handlers::handlers::get_dataset)),
            )
            .service(
                web::resource("/insert").route(web::post().to(handlers::handlers::insert_dataset)),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
