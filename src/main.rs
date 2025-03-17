use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

async fn hello() -> impl Responder {
    HttpResponse::Ok().json(json!({
        "message": "Hello from Rust!"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .route("/api/hello", web::get().to(hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}