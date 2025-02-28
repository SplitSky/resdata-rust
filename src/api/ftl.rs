use crate::model::datamodels::MyDocument;
use actix_web::{get, post, web, HttpResponse, Responder};
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Client;
use serde_json::json;

#[post("/insert")]
pub async fn insert_dataset(
    client: web::Data<Client>,
    json_body: web::Json<MyDocument>,
) -> impl Responder {
    let db = client.database("Database_Name");
    let collection = db.collection::<MyDocument>("my_collection");
}
