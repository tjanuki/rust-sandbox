use actix_web::{Error, dev::ServiceRequest};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use crate::auth::validate_token;

const TEST_SECRET: &[u8] = b"test_secret_key";

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let token = credentials.token();
    
    match validate_token(token, TEST_SECRET) {
        Ok(_claims) => Ok(req),
        Err(_) => Err((
            actix_web::error::ErrorUnauthorized("Invalid token"),
            req
        ))
    }
} 