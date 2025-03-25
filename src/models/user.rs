use crate::schema::users;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Clone, Queryable, Selectable)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    #[validate(length(min = 3, message = "Name must be at least 3 characters long"))]
    pub name: String,
    #[validate(email(message = "Invalid email address"))]
    pub email: String,
}