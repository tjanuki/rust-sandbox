use actix_web::{test, App};
use common::TestApp;
use rust_sandbox::{
    models::user::NewUser,
    auth::{create_token, Claims},
};
use jsonwebtoken::{decode, DecodingKey, Validation};

mod common;

const TEST_SECRET: &[u8] = b"test_secret_key";

#[actix_web::test]
async fn test_user_authentication_flow() {
    let test_app = TestApp::new();
    
    let app = test::init_service(
        App::new().configure(test_app.app_config())
    ).await;

    // 1. Create a new user
    let new_user = NewUser {
        name: "Auth Test User".to_string(),
        email: "auth_test@example.com".to_string(),
    };

    let create_req = test::TestRequest::post()
        .uri("/api/users")
        .set_json(&new_user)
        .to_request();
    
    let resp = test::call_service(&app, create_req).await;
    assert!(resp.status().is_success());
    
    let user: serde_json::Value = test::read_body_json(resp).await;
    let user_id = user["id"].as_i64().unwrap();

    // 2. Generate JWT token
    let token = create_token(user_id.to_string(), TEST_SECRET)
        .expect("Token creation should succeed");

    // 3. Verify protected endpoint access
    let protected_req = test::TestRequest::get()
        .uri("/api/users/protected")
        .insert_header(("Authorization", format!("Bearer {}", token)))
        .to_request();
    
    let resp = test::call_service(&app, protected_req).await;
    assert!(resp.status().is_success());

    // Clean up
    test_app.cleanup();
}

#[actix_web::test]
async fn test_invalid_token_access() {
    let test_app = TestApp::new();
    
    let app = test::init_service(
        App::new().configure(test_app.app_config())
    ).await;

    // Try accessing protected endpoint with invalid token
    let invalid_token = "invalid.token.here";
    let req = test::TestRequest::get()
        .uri("/api/users/protected")
        .insert_header(("Authorization", format!("Bearer {}", invalid_token)))
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401); // Unauthorized

    // Clean up
    test_app.cleanup();
}

#[actix_web::test]
async fn test_missing_token_access() {
    let test_app = TestApp::new();
    
    let app = test::init_service(
        App::new().configure(test_app.app_config())
    ).await;

    // Try accessing protected endpoint without token
    let req = test::TestRequest::get()
        .uri("/api/users/protected")
        .to_request();
    
    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), 401); // Unauthorized

    // Clean up
    test_app.cleanup();
}

#[actix_web::test]
async fn test_token_validation() {
    // Create a token
    let user_id = "123".to_string();
    let token = create_token(user_id.clone(), TEST_SECRET)
        .expect("Token creation should succeed");

    // Decode and validate the token
    let validation = Validation::default();
    let token_data = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(TEST_SECRET),
        &validation
    ).expect("Token should be valid");

    // Verify claims
    assert_eq!(token_data.claims.sub, user_id);
    assert!(token_data.claims.exp > token_data.claims.iat);
} 