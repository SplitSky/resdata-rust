use crate::model::data_structs::Dataset; // , Experiment, Project};
use futures::{TryStreamExt, stream::StreamExt};
use mongodb::{Client, Collection, bson::Document, bson::doc, options::ClientOptions};

#[derive(Clone)]
pub struct Db {
    pub collection: Collection<Dataset>,
}

impl Db {
    pub async fn new(connection_string: &str) -> Self {
        let client_options = ClientOptions::parse(connection_string).await.unwrap();
        let client = Client::with_options(client_options).unwrap();
        let database = client.database("test_database");
        let collection = database.collection("test_collection");
        Db { collection }
    }

    pub async fn insert_dataset(&self, doc: Dataset) -> mongodb::error::Result<()> {
        return self.collection.insert_one(doc, None).await.map(|_| ());
    }

    pub async fn get_dataset(&self, id: &str) -> mongodb::error::Result<Option<Dataset>> {
        return self.collection.find_one(doc! {"id": id}, None).await;
    }

    pub async fn list_datasets(&self) -> mongodb::error::Result<Vec<Dataset>> {
        let cursor = self.collection.find(None, None).await?;
        let results: Vec<Dataset> = cursor.try_collect().await?;
        Ok(results)
    }
}
