use crate::models::user::{CreateUserRequest, User, UpdateUserRequest};
use crate::state::AppState;
use anyhow::Result;
use std::sync::Arc;
use bcrypt::{hash, DEFAULT_COST};
use crate::db::get_db;

pub async fn create_user(user: CreateUserRequest) -> Result<User> {
    let pool = get_db().await?;
    let hashed_password = hash(user.password, DEFAULT_COST)?;
    let new_user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email, password, role) VALUES ($1, $2, $3, $4) RETURNING *",
        user.name,
        user.email,
        hashed_password,
        user.role
    )
    .fetch_one(&pool)
    .await?;
    Ok(new_user)
}

pub async fn get_user(id: i32) -> Result<User> {
    let pool = get_db().await?;
    let user = sqlx::query_as!(User, "SELECT id, name, email, password, role FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await?;
    Ok(user)
}

pub async fn get_all_users() -> Result<Vec<User>> {
    let pool = get_db().await?;
    let users = sqlx::query_as!(User, "SELECT id, name, email, password, role FROM users")
        .fetch_all(&pool)
        .await?;
    Ok(users)
}

pub async fn update_user(id: i32, name: String, email: String) -> Result<User> {
    let pool = get_db().await?;
    let user = sqlx::query_as!(
        User,
        "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING id, name, email, password, role",
        name,
        email,
        id
    )
    .fetch_one(&pool)
    .await?;
    Ok(user)
}