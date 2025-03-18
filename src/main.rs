mod models;
mod routes;

use actix_web::{web, App, HttpServer};
use routes::users::{create_user, get_user, get_users, init_users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting server at http://localhost:8080");

    let app_state = web::Data::new(init_users());

    HttpServer::new(move || {
        App::new()
        .app_data(app_state.clone())
        .route("/api/users", web::get().to(get_users))
        .route("/api/users/{id}", web::get().to(get_user))
        .route("/api/users", web::post().to(create_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}