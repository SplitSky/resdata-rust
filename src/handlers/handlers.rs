use crate::middleware::db::Db;
use crate::model::data_structs::Dataset;
use actix_web::{HttpResponse, Responder, web};
use log::error;

fn handle_error<E: std::fmt::Debug>(error: E, message: &str) -> HttpResponse {
    error!("{}: {:?}", message, error);
    HttpResponse::InternalServerError().json(message)
}

pub async fn insert_dataset(db: web::Data<Db>, item: web::Json<Dataset>) -> impl Responder {
    match db.insert_dataset(item.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Document inserted"),
        Err(e) => handle_error(e, "Failed to insert document"),
    }
}

pub async fn get_dataset(db: web::Data<Db>, path: web::Path<String>) -> impl Responder {
    match db.get_dataset(&path.into_inner()).await {
        Ok(Some(doc)) => HttpResponse::Ok().json(doc),
        Ok(None) => HttpResponse::NotFound().json("Document not found"),
        Err(e) => handle_error(e, "Failed to fetch document"),
    }
}

pub async fn list_datasets(db: web::Data<Db>) -> impl Responder {
    match db.list_datasets().await {
        Ok(docs) => HttpResponse::Ok().json(docs),
        Err(e) => handle_error(e, "Failed to fetch datasets"),
    }
}
