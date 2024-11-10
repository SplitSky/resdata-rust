use crate::model::datamodels::MyDocument;
use actix_web::{get, web, HttpResponse, Responder};
use mongodb::bson::{doc, oid::ObjectId};
use mongodb::Client;

#[get("/get/{id}")]
pub async fn get_document(client: web::Data<Client>, path: web::Path<String>) -> impl Responder {
    let db = client.database("your_database_name");
    let collection = db.collection::<MyDocument>("my_collection");

    let object_id = match ObjectId::parse_str(path.as_str()) {
        // path.to_str() -> this is to not double reference
        // the path string
        Ok(id) => id,
        Err(_) => return HttpResponse::BadRequest().body("Invalid ID format"),
    };

    // Query the document by id
    match collection.find_one(doc! {"_id": object_id}, None).await {
        Ok(Some(document)) => HttpResponse::Ok().json(document),
        Ok(None) => HttpResponse::NotFound().body("Document not found"),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to retrieve document: {}", e))
        }
    }
}
