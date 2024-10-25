use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    return "Hello World";
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index) // Attach the index value
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

