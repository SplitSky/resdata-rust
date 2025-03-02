use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Dataset {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>, // Mongo auto id
    data: Vec<i64>,
    headers: Vec<String>,
    author: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Experiment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>, // Mongo auto id
    data: Vec<i64>,
    headers: Vec<String>,
    author: Vec<String>,
    datasets: Vec<Option<ObjectId>>,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    id: Option<ObjectId>, // Mongo auto id
    data: Vec<i64>,
    headers: Vec<String>,
    author: Vec<String>,
    experiments: Vec<Option<ObjectId>>,
}
