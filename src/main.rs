mod db;
mod app_config;
mod models;
mod services;
mod handlers;
mod state;

use app_config::get_config;
use db::connect_db;
use std::sync::Arc;
use state::AppState;
use handlers::{user_handler, order_handler};
use tracing_subscriber::FmtSubscriber;
use tower_http::trace::TraceLayer;
use axum::{Router, routing::{post, get, put}, http::StatusCode, extract::Extension};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Initialize tracing with more detailed configuration
    let subscriber = FmtSubscriber::builder()
        .with_max_level(tracing::Level::DEBUG)
        .with_target(false)
        .with_level(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .pretty()
        .init();

    let config = get_config();
    tracing::info!("Starting server on port {}", config.port);

    let pool = connect_db(&config.database_url).await.unwrap();
    let app_state = Arc::new(AppState { pool });

    let router = Router::new()
        .route("/health", get(|| async { 
            tracing::info!("Health check requested");
            (StatusCode::OK, "OK")
        }))
        // .route("/users", get(user_handler::get_all_users))
        .nest("/users", Router::new()
            .route("/", post(user_handler::create_user))
            .route("/", get(user_handler::get_all_users))
            .route("/{id}", get(user_handler::get_user))
            .route("/{id}", put(user_handler::update_user))
        )
        .nest("/orders", Router::new()
            .route("/", post(order_handler::create_order))
            .route("/{id}", get(order_handler::get_order))
            .route("/user/{id}", get(order_handler::get_user_orders))
        )
        .layer(Extension(app_state))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &axum::http::Request<_>| {
                    tracing::info_span!(
                        "request",
                        method = %request.method(),
                        uri = %request.uri(),
                        version = ?request.version(),
                    )
                })
                .on_request(|request: &axum::http::Request<_>, _span: &tracing::Span| {
                    tracing::info!("started processing request");
                })
                .on_response(|response: &axum::http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                    tracing::info!("finished processing request (latency: {:?})", latency);
                })
        );

    let addr = format!("0.0.0.0:{}", config.port).parse::<SocketAddr>().unwrap();
    tracing::info!("Listening on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(router.into_make_service())
        .await
        .unwrap();

    // Server::bind(&format!("0.0.0.0:{}", CONFIG.port).parse().unwrap())
    //     .serve(router)
    //     .await
    //     .unwrap();

    // let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", CONFIG.port)).await.unwrap();
    // axum::serve(listener, router).await.unwrap();
}
