use actix_web::{web};
use rust_sandbox::{
    db::DbPool,
    routes::users::{create_user, delete_user, get_user, get_users, get_protected_resource},
};
use rust_sandbox::test_utils::{establish_test_connection, init_test_env, clean_database};
use rust_sandbox::middleware::auth::validator;
use actix_web_httpauth::middleware::HttpAuthentication;

pub struct TestApp {
    pub pool: web::Data<DbPool>,
}

impl TestApp {
    pub fn new() -> Self {
        init_test_env();
        let pool = web::Data::new(establish_test_connection());
        Self { pool }
    }

    pub fn cleanup(&self) {
        clean_database(&self.pool);
    }

    pub fn app_config(&self) -> impl FnOnce(&mut web::ServiceConfig) {
        let pool = self.pool.clone();
        let auth = HttpAuthentication::bearer(validator);
        
        move |cfg: &mut web::ServiceConfig| {
            cfg.app_data(pool)
                .service(
                    web::resource("/api/users/protected")
                        .wrap(auth)
                        .route(web::get().to(get_protected_resource))
                )
                .route("/api/users", web::get().to(get_users))
                .route("/api/users/{id}", web::get().to(get_user))
                .route("/api/users", web::post().to(create_user))
                .route("/api/users/{id}", web::delete().to(delete_user));
        }
    }
} 