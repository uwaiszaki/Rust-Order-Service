use axum::{
    extract::Path,
    http::StatusCode,
    Json,
};
use crate::models::order::{CreateOrderRequest, Order};
use crate::services::order_service;
use crate::handlers::auth_handler::{AuthUser, AuthError};
use anyhow::Result;

pub async fn create_order(
    auth_user: AuthUser,
    Json(order): Json<CreateOrderRequest>,
) -> Result<Json<Order>, StatusCode> {
    // Ensure user can only create orders for themselves
    if auth_user.user_id != order.user_id {
        return Err(StatusCode::FORBIDDEN);
    }
    
    order_service::create_order(order)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_orders(auth_user: AuthUser) -> Result<Json<Vec<Order>>, StatusCode> {
    order_service::get_all_orders()
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_user_orders(
    auth_user: AuthUser,
    Path(user_id): Path<i32>,
) -> Result<Json<Vec<Order>>, StatusCode> {
    // Ensure user can only view their own orders unless they're admin
    if auth_user.user_id != user_id && auth_user.role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }
    
    order_service::get_user_orders(user_id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}