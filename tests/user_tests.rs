mod common;

use actix_web::{test, App};
use common::TestApp;
use rust_sandbox::models::user::NewUser;

#[actix_web::test]
async fn test_create_and_get_user() {
    let test_app = TestApp::new();
    
    let app = test::init_service(
        App::new().configure(test_app.app_config())
    ).await;

    // Create test user
    let new_user = NewUser {
        name: "Test User".to_string(),
        email: "test@example.com".to_string(),
    };

    let req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(&new_user)
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    // Clean up after test
    test_app.cleanup();
}

#[actix_web::test]
async fn test_get_users_empty_list() {
    let test_app = TestApp::new();
    
    // Clean up the database before the test
    test_app.cleanup();
    
    let app = test::init_service(
        App::new().configure(test_app.app_config())
    ).await;

    let req = test::TestRequest::get()
        .uri("/api/users")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());

    let body: Vec<serde_json::Value> = test::read_body_json(resp).await;
    assert!(body.is_empty());

    // Clean up after test
    test_app.cleanup();
} 