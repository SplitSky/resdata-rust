use core::models::datamodels::MyDocument;

use reqwest::Client;
use std::error::Error;
use std::time::Duration; // Replace `your_crate_name` with the crate name from Cargo.toml

#[tokio::test]
async fn test_insert_and_get_document() -> Result<(), Box<dyn Error>> {
    // Base URL for your local server
    let base_url = "http://localhost:8080"; // Adjust the port if necessary
    let client = Client::new();

    // Step 1: Insert a new document
    let test_document = MyDocument {
        id: None, // MongoDB will generate the ID
        name: "Test Document".to_string(),
        description: "This is a test document.".to_string(),
    };

    let insert_resp = client
        .post(&format!("{}/insert", base_url))
        .json(&test_document)
        .send()
        .await?;

    assert!(insert_resp.status().is_success(), "Insert request failed");

    // Extract inserted ID from the response
    let insert_body: serde_json::Value = insert_resp.json().await?;
    let inserted_id = insert_body["inserted_id"]
        .as_str()
        .expect("Missing 'inserted_id'")
        .to_string();

    // Step 2: Retrieve the document by ID
    // Wait briefly to ensure the document is available for querying
    tokio::time::sleep(Duration::from_millis(500)).await;

    let get_resp = client
        .get(&format!("{}/get/{}", base_url, inserted_id))
        .send()
        .await?;

    assert!(get_resp.status().is_success(), "Get request failed");

    // Parse the response as MyDocument
    let retrieved_document: MyDocument = get_resp.json().await?;

    // Step 3: Compare the retrieved document with the original test document
    let expected_document = MyDocument {
        id: Some(inserted_id.clone()),
        name: test_document.name.clone(),
        description: test_document.description.clone(),
    };

    assert_eq!(
        retrieved_document, expected_document,
        "Retrieved document does not match the inserted document"
    );

    Ok(())
}
