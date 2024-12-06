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
    let db = client.database("Your_database_name");
    let collection = db.collection::<MyDocument>("my_collection");

    let new_doc = MyDocument {
        id: None, // TODO: implement in all of them Euan trick
        ..json_body.into_inner()
    };

    match collection.insert_one(new_doc, None).await {
        Ok(result) => HttpResponse::Ok().json(json!({"inserted_id": result.inserted_id})),
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("Failed to insert document: {}", e))
        }
    }
}

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

// #[get("/health")]
//pub async fn list_collections_api(client: web::Data<Client>, path: web::Path<String>)
// -> impl Responder {
//    let db = client.database()
// }