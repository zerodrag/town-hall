mod handlers;
mod state;

use std::sync::Arc;

pub use state::AppState;

use anyhow::Result;
use axum::routing::*;

use handlers::*;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?; // Export .env
    tracing_subscriber::fmt::init(); // Print tracing::_

    let state = Arc::new(AppState::new().await?);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app(state))
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

fn app(state: Arc<AppState>) -> axum::Router {
    axum::Router::new()
        .fallback(fallback)
        .route("/api", get(hello_world))
        .route("/api/health", get(health))
        .route("/api/users/{id}", get(get_user_from_id))
        .route("/api/quests/{id}", get(get_quest_from_id))
        .route("/auth/github", get(github_login))
        .route("/auth/github/callback", get(github_callback))
        .with_state(state)
}
