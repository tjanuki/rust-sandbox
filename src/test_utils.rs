use crate::db::{DbPool, DbConnection};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize test environment
pub fn init_test_env() {
    INIT.call_once(|| {
        // You could set up logging or other one-time initialization here
        std::env::set_var("DATABASE_URL", ":memory:"); // Use in-memory SQLite for tests
    });
}

/// Create a new database pool for testing
pub fn establish_test_connection() -> DbPool {
    let manager = ConnectionManager::<DbConnection>::new(":memory:");
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create test connection pool");

    // Run migrations on the test database
    let conn = &mut pool.get().expect("Failed to get connection");
    run_migrations(conn);
    
    pool
}

/// Run migrations for test database
fn run_migrations(conn: &mut DbConnection) {
    use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
    
    const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");
    conn.run_pending_migrations(MIGRATIONS).expect("Failed to run migrations");
}

/// Clean up test data after each test
pub fn clean_database(pool: &DbPool) {
    use crate::schema::users::dsl::*;
    let conn = &mut pool.get().expect("Failed to get connection");
    diesel::delete(users)
        .execute(conn)
        .expect("Failed to clean test database");
}

/// Helper function to create a test user
pub fn create_test_user(pool: &DbPool, name: &str, email: &str) -> crate::models::user::User {
    use crate::models::user::NewUser;
    use crate::schema::users;

    let conn = &mut pool.get().expect("Failed to get connection");
    let new_user = NewUser {
        name: name.to_string(),
        email: email.to_string(),
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Failed to create test user");

    users::table
        .filter(users::email.eq(email))
        .first(conn)
        .expect("Failed to fetch created test user")
} 