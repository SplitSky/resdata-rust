use crate::model::data_structs::Dataset; // , Experiment, Project};
use futures::TryStreamExt;
use mongodb::{
    Client, Collection,
    bson::{doc, oid::ObjectId},
    options::ClientOptions,
};

#[derive(Clone)]
pub struct Db {
    pub collection: Collection<Dataset>,
}

impl Db {
    pub async fn new(
        connection_string: &str,
        database_name: &str,
        collection_name: &str,
    ) -> mongodb::error::Result<Self> {
        let client_options = ClientOptions::parse(connection_string).await?;
        // client_options.tls = Some(mongodb::options::Tls::Enabled(TlsOptions::default())); // TODO: check whethere this is necessary -> if it is also make previous mutable
        let client = Client::with_options(client_options)?;
        let database = client.database(database_name);
        let collection = database.collection(collection_name);
        Ok(Db { collection })
    }

    pub async fn insert_dataset(&self, doc: Dataset) -> mongodb::error::Result<()> {
        self.collection.insert_one(doc, None).await.map(|_| ())
    }

    pub async fn get_dataset(&self, id: &str) -> mongodb::error::Result<Option<Dataset>> {
        let object_id = ObjectId::parse_str(id)
            .map_err(|e| mongodb::error::Error::custom(format!("Invalid ObjectId: {}", e)))?;
        self.collection
            .find_one(doc! {"_id": object_id}, None)
            .await
    }

    pub async fn list_datasets(&self) -> mongodb::error::Result<Vec<Dataset>> {
        let cursor = self.collection.find(None, None).await?;
        let results: Vec<Dataset> = cursor.try_collect().await?;
        Ok(results)
    }
}
