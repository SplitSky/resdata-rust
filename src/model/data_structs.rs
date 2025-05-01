use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Dataset {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // Mongo auto id
    pub data: Vec<i64>,
    pub headers: Vec<String>,
    pub author: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Experiment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // Mongo auto id
    pub data: Vec<i64>,
    pub headers: Vec<String>,
    pub author: Vec<String>,
    pub datasets: Vec<Option<ObjectId>>,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // Mongo auto id
    pub data: Vec<i64>,
    pub headers: Vec<String>,
    pub author: Vec<String>,
    pub experiments: Vec<Option<ObjectId>>,
}
