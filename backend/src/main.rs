mod database;
mod handlers;

use std::sync::Arc;

pub use database::Database;

use anyhow::Result;
use axum::routing::*;

use handlers::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?; // Export .env
    tracing_subscriber::fmt::init(); // Print tracing::_
    let state = Arc::new(Database::new().await?);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app(state))
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

fn app(state: Arc<Database>) -> axum::Router {
    let api = axum::Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health))
        .route("/users", post(create_user))
        .route("/users/{id}", get(get_user_from_id))
        .route("/quests/{id}", get(get_quest_from_id))
        .with_state(state);
    axum::Router::new().fallback(fallback).nest("/api", api)
}
