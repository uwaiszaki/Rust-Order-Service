use crate::models::user::{CreateUserRequest, User, UpdateUserRequest};
use crate::state::AppState;
use anyhow::Result;
use std::sync::Arc;
use bcrypt::{hash, DEFAULT_COST};

pub async fn create_user(state: Arc<AppState>, user: CreateUserRequest) -> Result<User> {
    let hashed_password = hash(user.password, DEFAULT_COST)?;
    let new_user = sqlx::query_as!(
        User, 
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING *", 
        user.name, 
        user.email,
        hashed_password
    )
    .fetch_one(&state.pool)
    .await?;
    Ok(new_user)
}

pub async fn get_user(state: Arc<AppState>, id: i32) -> Result<Option<User>> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(&state.pool)
        .await?;
    Ok(user)
}

pub async fn get_all_users(state: Arc<AppState>) -> Result<Vec<User>> {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(&state.pool)
        .await?;
    Ok(users)
}

pub async fn update_user(state: Arc<AppState>, id: i32, user: UpdateUserRequest) -> Result<User> {
    let user = sqlx::query_as!(User, "UPDATE users SET name = $1, email = $2 WHERE id = $3 RETURNING *", user.name, user.email, id)
        .fetch_one(&state.pool)
        .await?;
    Ok(user)
}