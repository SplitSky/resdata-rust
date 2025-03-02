use mongodb::{
    Client,
    bson::doc,
    options::{ClientOptions, ServerApi, ServerApiVersion},
};

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    let mut client_options = ClientOptions::parse(
        "mongodb+srv://splitsky:wombatcombat1@cluster0.xfvstgi.mongodb.net/?retryWrites=true&w=majority&appName=Cluster0",
    ).await.unwrap();
    println!("client options done");
    let server_api = ServerApi::builder().version(ServerApiVersion::V1).build();
    client_options.server_api = Some(server_api);
    println!("client options deployed");
    let client = Client::with_options(client_options).unwrap();

    println!("client made");
    client
        .database("admin")
        .run_command(doc! {"ping": 10}, None)
        .await
        .unwrap();
    println!("pinged deployment");
    Ok(())
}
