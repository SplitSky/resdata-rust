use crate::MyDocument;
use actix_web::{post, web, HttpResponse, Responder};
use mongodb::bson::doc;
use mongodb::Client;
use crate::MyDocument

#[post("/insert")]
pub async fn insert_dataset(client: web::Data<Client>,json_body: web::Json<MyDocument>,) -> impl Responder {
    let db = client.databset("Your_database_name");
    let collection = db.collection::<MyDocument>("my_collection");
    let new_doc = MyDocument{
        id: None,
        name: json_body.name.clone(),
        description: json_body.description.clone(),
    };
    match collection.insert_one(new_doc, None).await {
        Ok(result) => HttpResponse::Ok().json(!doc {"inserted_id": result.inserted_id}),
        Err(e) => HttpResponse::InternalServerError().body(format!("Failed to insert document: {}", e)),
    }
}


