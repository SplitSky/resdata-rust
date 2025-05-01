use resdata_rust::middleware::db::Db;
use resdata_rust::model::data_structs::Dataset;
use tokio;

#[tokio::test]
async fn test_insert_and_get_dataset() {
    let db = Db::new("mongodb://localhost:27017", "test_db", "test_collection")
        .await
        .unwrap();

    let dataset = Dataset {
        id: None,
        data: vec![10, 20, 30],
        headers: vec!["Header1".to_string()],
        author: vec!["Author1".to_string()],
    };

    db.insert_dataset(dataset.clone()).await.unwrap();

    let inserted_dataset = db
        .list_datasets()
        .await
        .unwrap()
        .into_iter()
        .find(|d| d.data == dataset.data)
        .unwrap();

    assert_eq!(inserted_dataset.headers, dataset.headers);
    assert_eq!(inserted_dataset.author, dataset.author);

    let fetched_dataset = db
        .get_dataset(&inserted_dataset.id.unwrap().to_hex())
        .await
        .unwrap()
        .unwrap();

    assert_eq!(fetched_dataset.data, dataset.data);
}
