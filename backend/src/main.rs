pub mod handlers;
mod router;
mod state;

use anyhow::Result;

use http::{HeaderValue, Method, header};
use sqlx::PgPool;
pub use state::AppState;
use tower_http::cors::CorsLayer;
use tower_sessions::{SessionManagerLayer, cookie::SameSite};
use tower_sessions_sqlx_store::PostgresStore;

use crate::handlers::shutdown_signal;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?; // Export .env
    tracing_subscriber::fmt::init(); // Print tracing::_

    let state = AppState::new().await?;
    let session_layer = session_layer(state.db_pool.clone()).await?;
    let cors_layer = cors_layer().await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    let app = router::root()
        .await?
        .with_state(state)
        .layer(session_layer)
        .layer(cors_layer);
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

async fn cors_layer() -> Result<CorsLayer> {
    let layer = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true);
    Ok(layer)
}
