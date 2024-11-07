// ftl handling functions

// use crates here

use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse, Responder,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};
//use std::fmt::{Display, Debug};

#[derive(Deserialize, Serialize)]
pub struct TaskIdentifier {
    task_global_id: String,
}

#[derive(Deserialize)]
pub struct TaskCompletionRequest {
    result_file: String,
}

#[derive(Deserialize)]
pub struct SubmitTaskRequest {
    user_id: String,
    task_type: String,
    source_file: String,
}

#[derive(Debug, Display)]
pub enum TaskError {
    TaskNotFound,
    TaskUpdateFailure,
    TaskCreationFailure,
    BadTaskRequest,
}

impl ResponseError for TaskError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::TaskNotFound => StatusCode::NOT_FOUND,
            TaskError::TaskUpdateFailure => StatusCode::FAILED_DEPENDENCY,
            TaskError::TaskCreationFailure => StatusCode::FAILED_DEPENDENCY,
            TaskError::BadTaskRequest => StatusCode::BAD_REQUEST,
        }
    }
}

#[derive(Debug, Display, Serialize, Deserialize)]
pub enum HealthError {
    MongoDown,
    MongoRunning,
}
impl ResponseError for HealthError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }
}

#[derive(Serialize)]
struct HealthStatus {
    mongo_version: String,
    status: HealthError,
}

#[get("/healthcheck")]
pub async fn healthcheck() -> HttpResponse {
    let response = HealthStatus {
        mongo_version: "thing".to_string(),
        status: HealthError::MongoDown,
    };
    return HttpResponse::Ok().json(response);
}
