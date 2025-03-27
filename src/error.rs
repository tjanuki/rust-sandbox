use std::error::Error;
use std::fmt;
use serde::Serialize;

// Define a trait for API errors
pub trait ApiError: Error {
    fn error_code(&self) -> &str;
    fn status_code(&self) -> u16;
}

// Add #[derive(Serialize)] to make UserError serializable
#[derive(Debug, Serialize)]
pub struct UserError {
    pub code: String,
    pub message: String,
    pub status: u16,
}

// Implement standard error traits
impl fmt::Display for UserError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl Error for UserError {}

// Implement our custom trait
impl ApiError for UserError {
    fn error_code(&self) -> &str {
        &self.code
    }

    fn status_code(&self) -> u16 {
        self.status
    }
} 