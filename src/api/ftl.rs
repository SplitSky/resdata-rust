use crate::api::models::Dataset; // data structures
use actix_web::{get, post, web, HttpResponse, Responder};
use futures::task::waker;
use serde_json::json;
use mongodb::{bson::doc, options::{ClientOptions, ServerApi, ServerApiVersion}, Client};

#[post("/insert")]
pub async fn insert(payload: web::Bytes) -> impl Responder {
    // Deserialize JSON manually using Serde
    let dataset: Dataset = match serde_json::from_slice(&payload) {
        Ok(data) => data,
        Err(_) => return HttpResponse::BadRequest().body("Invalid JSON"),
    };

    // Placeholder for database insertion logic
    println!("Inserting dataset with id: {}", dataset.id);

    // Manually serialize the dataset to JSON for the response
    match serde_json::to_string(&dataset) {
        Ok(json_data) => HttpResponse::Ok().body(json_data),
        Err(_) => HttpResponse::InternalServerError().body("Failed to serialize response"),
    }
}

#[get("/fetch/{id}")]
pub async fn fetch(id: web::Path<u32>) -> impl Responder {
    let dataset_id = id.into_inner();

    // Placeholder for database fetching logic
    println!("Fetching dataset with id: {}", dataset_id);

    // Create a dummy dataset
    let dummy_dataset = Dataset {
        id: dataset_id,
        name: "Sample Dataset".to_string(),
        data: "This is sample data".to_string(),
    };

    // Manually serialize the dataset to JSON for the response
    match serde_json::to_string(&dummy_dataset) {
        Ok(json_data) => HttpResponse::Ok().body(json_data),
        Err(_) => HttpResponse::InternalServerError().body("Failed to serialize response"),
    }
}

#[get("/healthcheck")]
pub async fn healthcheck() -> impl Responder {
    // Placeholder for health check logic (e.g., ping database or services)
    // Here, we'll just return a simple JSON status
    
    // sample mongo code start
    
    
    
    #[tokio::main]
    async fn main() -> mongodb::error::Result<()> {
        let mut connection_string =  "mongodb+srv://splitsky:<db_password>@cluster0.xfvstgi.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0";
        ClientOptions::parse(connection_string);
          $.await?;
    
      // Set the server_api field of the client_options object to set the version of the Stable API on the client
      let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
      client_options.server_api = Some(server_api);
    
      // Get a handle to the cluster
      let client = Client::with_options(client_options)?;
    
      // Ping the server to see if you can connect to the cluster
      client
        .database("admin")
        .run_command(doc! {"ping": 1}, None)
        .await?;
      println!("Pinged your deployment. You successfully connected to MongoDB!");
    
      Ok(())
    }
    // sample mongo code end


    // Manually create a JSON response using serde_json::json macro
    let health_status = json!({
        "status": "healthy",
        "database_connection": true,
        "other_service_status": "operational"
    });

    match serde_json::to_string(&health_status) {
        Ok(json_data) => HttpResponse::Ok().body(json_data),
        Err(_) => HttpResponse::InternalServerError().body("Failed to serialize health status"),
    }
}
