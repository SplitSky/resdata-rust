use crate::middleware::db::Db;
use crate::model::data_structs::Dataset;
use actix_web::{HttpResponse, Responder, web};

pub async fn insert_dataset(db: web::Data<Db>, item: web::Json<Dataset>) -> impl Responder {
    match db.insert_dataset(item.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Document inserted"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to insert document"),
    }
}

pub async fn get_dataset(db: web::Data<Db>, path: web::Path<String>) -> impl Responder {
    match db.get_dataset(&path.into_inner()).await {
        Ok(Some(doc)) => HttpResponse::Ok().json(doc),
        Ok(None) => HttpResponse::NotFound().json("Document not found"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch document"),
    }
}

pub async fn list_datasets(db: web::Data<Db>, path: web::Path<String>) -> impl Responder {
    match db.list_datasets().await {
        Ok(Vec<Dataset>) => 



        Ok(Some(doc)) => HttpResponse::Ok().json(doc),
        Ok(None) => HttpResponse::NotFound().json("Document not found"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch document"),
    }
}
