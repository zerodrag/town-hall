pub mod handlers;
mod router;
mod state;

use anyhow::Result;

use sqlx::PgPool;
pub use state::AppState;
use tower_sessions::{SessionManagerLayer, cookie::SameSite};
use tower_sessions_sqlx_store::PostgresStore;

use crate::handlers::shutdown_signal;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?; // Export .env
    tracing_subscriber::fmt::init(); // Print tracing::_

    let state = AppState::new().await?;
    let session_layer = session_layer(state.db_pool.clone()).await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    let app = router::root().await?.with_state(state).layer(session_layer);
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}

async fn session_layer(db_pool: PgPool) -> Result<SessionManagerLayer<PostgresStore>> {
    let session_store = PostgresStore::new(db_pool);
    session_store.migrate().await?;
    let layer = SessionManagerLayer::new(session_store).with_same_site(SameSite::Lax);
    Ok(layer)
}
