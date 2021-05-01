use sqlx::{FromRow};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, FromRow, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateUserRequest {
    pub name: String,
    pub email: String,
}
