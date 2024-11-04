use crate::models::Dataset;
use crate::MongoRepo;
use actix_web::{web, HttpResponse, Responder};

pub async fn health_check(repo: web::Data<MongoRepo>) -> impl Responder {
    match repo.health_check().await {
        Ok(health) => HttpResponse::Ok().json(health),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn insert_dataset(
    repo: web::Data<MongoRepo>,
    dataset: web::Json<Dataset>,
) -> impl Responder {
    match repo.insert_dataset(dataset.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Dataset inserted successfully"),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn fetch_dataset(repo: web::Data<MongoRepo>, id: web::Path<String>) -> impl Responder {
    match repo.fetch_dataset(&id).await {
        Ok(Some(dataset)) => HttpResponse::Ok().json(dataset),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
