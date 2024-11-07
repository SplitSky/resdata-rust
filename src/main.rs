mod api;
mod model;
mod repository;

use actix_web::{middleware::Logger, web::Data, App, HttpServer};
use api::ftl::healthcheck; // api functions
use mongodb::{
    bson::doc,
    //options::{ClientOptions, ServerApi, ServerApiVersion},
    //Client,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    // MongoDB configuration
    // Initialize MongoRepository
    // let mongo_repo = MongoRepository::new(mongo_client, String::from("task"));
    // let mongo_data = Data::new(mongo_repo);

    // NOTE: Working MongoDB client call
    //
    // let mut client_options =
    //  .await;
    // Set the server_api field of the client_options object to set the version of the Stable API on the client
    // let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    // client_options.server_api = Some(server_api);
    // Get a handle to the cluster
    // let client = Client::with_options(client_options);
    // Ping the server to see if you can connect to the cluster
    // client
    //.database("admin")
    //  .run_command(doc! {"ping": 1}, None)
    //  .await?;
    //println!("Pinged your deployment. You successfully connected to MongoDB!");
    //
    //
    //
    // Set up the Actix server with MongoDB repository as shared state
    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .wrap(logger)
            //.app_data(mongo_data.clone()) // MongoDB shared state
            .service(healthcheck)
        // .service() // add more calls
    })
    .bind(("127.0.0.1", 8081))?
    .run()
    .await
}
