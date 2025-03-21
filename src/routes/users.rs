use crate::db::DbPool;
use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::*;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;

pub async fn get_users(pool: web::Data<DbPool>) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error"),
    };

    let result = users.load::<User>(&mut conn);

    match result {
        Ok(users_list) => HttpResponse::Ok().json(users_list),
        Err(_) => HttpResponse::InternalServerError().body("Error loading users"),
    }
}

pub async fn get_user(path: web::Path<i32>, pool: web::Data<DbPool>) -> impl Responder {
    let user_id = path.into_inner();
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error"),
    };

    let result = users.find(user_id).first::<User>(&mut conn);

    match result {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(diesel::NotFound) => HttpResponse::NotFound().body("User not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error loading user"),
    }
}

pub async fn create_user(
    new_user: web::Json<NewUser>,
    pool: web::Data<DbPool>,
) -> impl Responder {
    let mut conn = match pool.get() {
        Ok(conn) => conn,
        Err(_) => return HttpResponse::InternalServerError().body("Database connection error"),
    };

    // Extract email before consuming new_user
    let user_email = new_user.email.clone();

    // Execute insert and then fetch the user separately
    let result = diesel::insert_into(users)
        .values(&new_user.into_inner())
        .execute(&mut conn);

    match result {
        Ok(_) => {
            // Fetch the newly created user by email (since it's unique)
            match users.filter(email.eq(user_email)).first::<User>(&mut conn) {
                Ok(user) => HttpResponse::Created().json(user),
                Err(_) => HttpResponse::InternalServerError().body("Error fetching created user"),
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error creating user"),
    }
}