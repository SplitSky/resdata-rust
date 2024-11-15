use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyDocument {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, //mongoDB automatically assigns an _id field
    pub name: String,
    pub description: String,
}

// NOTE: Datastructures to implement
// Dataset
// Experiment
// Project
//
