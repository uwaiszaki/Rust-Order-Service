use crate::models::user::{CreateUserRequest, UpdateUserRequest, User};
use crate::state::AppState;
use crate::services::user_service;
use axum::http::StatusCode;
use axum::{
    extract::{Path, Extension},
    Json,
};
use std::sync::Arc;

pub async fn create_user(
    Extension(state): Extension<Arc<AppState>>,
    Json(user): Json<CreateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    let name = user.name.clone();
    user_service::create_user(state, user)
        .await
        .map(Json)
        .map_err(|err| {
            tracing::error!("Failed to create user {}: {}", name, err);
            StatusCode::INTERNAL_SERVER_ERROR
        })
}

pub async fn get_all_users(
    Extension(state): Extension<Arc<AppState>>,
) -> Result<Json<Vec<User>>, StatusCode> {
    user_service::get_all_users(state)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_user(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    user_service::get_user(state, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
        .and_then(|user| user.ok_or(StatusCode::NOT_FOUND))
        .map(Json)
}

pub async fn update_user(
    Extension(state): Extension<Arc<AppState>>,
    Path(id): Path<i32>,
    Json(user): Json<UpdateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    user_service::update_user(state, id, user)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
