mod handlers;
mod state;

use std::sync::Arc;

pub use state::AppState;

use anyhow::Result;
use axum::routing::*;
use tower_sessions::{SessionManagerLayer, cookie::SameSite};
use tower_sessions_sqlx_store::PostgresStore;

use handlers::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?; // Export .env
    tracing_subscriber::fmt::init(); // Print tracing::_

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    let app = app().await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn app() -> Result<axum::Router> {
    let state = Arc::new(AppState::new().await?);

    let session_store = PostgresStore::new(state.db_pool.clone());
    session_store.migrate().await?;
    let session_layer = SessionManagerLayer::new(session_store).with_same_site(SameSite::Lax);

    Ok(axum::Router::new()
        .fallback(fallback)
        .route("/api", get(hello_world))
        .route("/api/health", get(health))
        .route("/api/users/{id}", get(get_user_from_url))
        .route("/api/me", get(get_user_me))
        .route("/api/quests/{id}", get(get_quest_from_id))
        .route("/auth/github", get(github_login))
        .route("/auth/github/callback", get(github_callback))
        .with_state(state)
        .layer(session_layer))
}
