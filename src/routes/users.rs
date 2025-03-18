use crate::models::user::User;
use actix_web::{web, HttpResponse, Responder};
use std::sync::Mutex;

pub type Users = Mutex<Vec<User>>;

pub fn init_users() -> Users {
    Mutex::new(vec![
        User {
            id: Some(1),
            name: "John Doe".to_string(),
            email: "john.doe@example.com".to_string(),
        },
    ])
}

pub async fn get_users(data: web::Data<Users>) -> impl Responder {
    let users = data.lock().unwrap();
    HttpResponse::Ok().json(&*users )
}

pub async fn get_user(path: web::Path<u64>, data: web::Data<Users>) -> impl Responder {
    let user_id = path.into_inner();
    let users = data.lock().unwrap();

    if let Some(user) = users.iter().find(|user| user.id == Some(user_id)) {
        HttpResponse::Ok().json(user)
    } else {
        HttpResponse::NotFound().body("User not found")
    }
}

pub async fn create_user(user: web::Json<User>, data: web::Data<Users>) -> impl Responder {
    let mut users = data.lock().unwrap();

    let new_id = users.iter()
        .map(|user| user.id.unwrap_or(0))
        .max()
        .unwrap_or(0) + 1;

    let mut new_user = user.into_inner();
    new_user.id = Some(new_id);

    users.push(new_user.clone());
    HttpResponse::Created().json(new_user)
}
