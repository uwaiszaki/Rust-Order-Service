use crate::models::user::{CreateUserRequest, UpdateUserRequest, User};
use crate::state::AppState;
use crate::services::user_service;
use crate::handlers::auth_handler::{AuthUser, AdminUser};
use axum::http::StatusCode;
use axum::{
    extract::{Path, Extension},
    Json,
};
use std::sync::Arc;
use anyhow::Result;

pub async fn create_user(
    Json(user): Json<CreateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    user_service::create_user(user)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_users(
    admin: AdminUser,
) -> Result<Json<Vec<User>>, StatusCode> {
    user_service::get_all_users()
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_user(
    auth_user: AuthUser,
    Path(id): Path<i32>,
) -> Result<Json<User>, StatusCode> {
    // Users can only view their own profile unless they're admin
    if auth_user.user_id != id && auth_user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }
    
    user_service::get_user(id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::NOT_FOUND)
}

pub async fn update_user(
    auth_user: AuthUser,
    Path(id): Path<i32>,
    Json(user): Json<UpdateUserRequest>,
) -> Result<Json<User>, StatusCode> {
    // Users can only update their own profile unless they're admin
    if auth_user.user_id != id && auth_user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }
    
    user_service::update_user(id, user.name, user.email)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
