use crate::models::order::{CreateOrderRequest, Order};
use anyhow::Result;
use uuid::Uuid;
use std::sync::Arc;
use crate::state::AppState;

pub async fn create_order(state: Arc<AppState>, order: CreateOrderRequest) -> Result<Order> {
    let order = sqlx::query_as!(
        Order,
        "INSERT INTO orders (id, user_id, product_name, created_at) VALUES ($1, $2, $3, $4) RETURNING *",
        Uuid::new_v4(),
        order.user_id,
        order.product_name,
        chrono::Utc::now()
    )
    .fetch_one(&state.pool)
    .await?;

    Ok(order)
}

pub async fn get_order(state: Arc<AppState>, id: Uuid) -> Result<Order> {
    let order = sqlx::query_as!(Order, "SELECT id, user_id, product_name, created_at FROM orders WHERE id = $1", id)
        .fetch_one(&state.pool)
        .await?;
    Ok(order)
}

pub async fn get_user_orders(state: Arc<AppState>, user_id: i32) -> Result<Vec<Order>> {
    let orders = sqlx::query_as!(Order, "SELECT id, user_id, product_name, created_at FROM orders WHERE user_id = $1", user_id)
        .fetch_all(&state.pool)
        .await?;
    Ok(orders)
}
