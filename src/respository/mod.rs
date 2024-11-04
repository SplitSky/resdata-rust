use crate::models::{Dataset, HealthResponse};
use mongodb::{
    bson::{doc, Bson},
    Client, Collection,
};

pub struct MongoRepo {
    col: Collection<Dataset>,
}

impl MongoRepo {
    pub async fn init() -> Self {
        let client = Client::with_uri_str("mongodb://localhost:27017")
            .await
            .unwrap();
        let db = client.database("mydb");
        let col = db.collection::<Dataset>("datasets");
        MongoRepo { col }
    }

    pub async fn health_check(&self) -> Result<HealthResponse, Box<dyn std::error::Error>> {
        let cursor = self.col.find(None, None).await?;
        let datasets: Vec<Dataset> = cursor.try_collect().await?;
        Ok(HealthResponse {
            status: "Healthy".to_string(),
            datasets,
        })
    }

    pub async fn insert_dataset(&self, dataset: Dataset) -> Result<(), Box<dyn std::error::Error>> {
        self.col.insert_one(dataset, None).await?;
        Ok(())
    }

    pub async fn fetch_dataset(
        &self,
        id: &str,
    ) -> Result<Option<Dataset>, Box<dyn std::error::Error>> {
        let obj_id = ObjectId::with_string(id)?;
        let filter = doc! { "_id": obj_id };
        let dataset = self.col.find_one(filter, None).await?;
        Ok(dataset)
    }
}
