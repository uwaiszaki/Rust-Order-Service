use crate::models::order::{CreateOrderRequest, Order};
use crate::db::get_db;
use anyhow::Result;
use uuid::Uuid;

pub async fn create_order(order: CreateOrderRequest) -> Result<Order> {
    let pool = get_db().await?;
    let order = sqlx::query_as!(
        Order,
        "INSERT INTO orders (user_id, product_name) VALUES ($1, $2) RETURNING *",
        order.user_id,
        order.product_name
    )
    .fetch_one(&pool)
    .await?;
    Ok(order)
}

pub async fn get_all_orders() -> Result<Vec<Order>> {
    let pool = get_db().await?;
    let orders = sqlx::query_as!(Order, "SELECT * FROM orders")
        .fetch_all(&pool)
        .await?;
    Ok(orders)
}

pub async fn get_user_orders(user_id: i32) -> Result<Vec<Order>> {
    let pool = get_db().await?;
    let orders = sqlx::query_as!(
        Order,
        "SELECT * FROM orders WHERE user_id = $1",
        user_id
    )
    .fetch_all(&pool)
    .await?;
    Ok(orders)
}
