mod db;
mod models;
mod routes;
mod schema;

use actix_web::{web, App, HttpServer};
use routes::users::{create_user, get_user, get_users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Set up database connection pool
    let pool = db::establish_connection_pool();
    let db_data = web::Data::new(pool);

    println!("Starting server at http://localhost:8081");

    HttpServer::new(move || {
        App::new()
            .app_data(db_data.clone())
            .route("/api/users", web::get().to(get_users))
            .route("/api/users/{id}", web::get().to(get_user))
            .route("/api/users", web::post().to(create_user))
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}