use axum::{Json, extract::{Path, Extension}};
use std::sync::Arc;
use crate::state::AppState;
use crate::models::order::{CreateOrderRequest, Order};
use crate::services::order_service;
use uuid::Uuid;
use axum::http::StatusCode;

pub async fn create_order(Extension(state): Extension<Arc<AppState>>, Json(order): Json<CreateOrderRequest>) -> Result<Json<Order>, StatusCode> {
    order_service::create_order(state, order)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_order(Extension(state): Extension<Arc<AppState>>, Path(id): Path<Uuid>) -> Result<Json<Order>, StatusCode> {
    order_service::get_order(state, id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

pub async fn get_user_orders(Extension(state): Extension<Arc<AppState>>, Path(id): Path<i32>) -> Result<Json<Vec<Order>>, StatusCode> {
    order_service::get_user_orders(state, id)
        .await
        .map(Json)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}