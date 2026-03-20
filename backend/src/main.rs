pub mod handlers;
mod router;
mod state;

use anyhow::Result;

pub use crate::state::AppState;

const GENERATED_TS_TYPES_PATH: &'static str = "../frontend/src/lib/backend-types.ts";

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?; // Export .env
    tracing_subscriber::fmt::init(); // Print tracing::_

    generate_ts_types().await?;

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

async fn generate_ts_types() -> Result<()> {
    use specta_typescript::Typescript;

    let types = specta::collect();
    let types_str = Typescript::default()
        .bigint(specta_typescript::BigIntExportBehavior::String)
        .export(&types)?;
    tokio::fs::write(GENERATED_TS_TYPES_PATH, types_str).await?;
    Ok(())
}

use sqlx::PgPool;
use tower_sessions::{SessionManagerLayer, cookie::SameSite};
use tower_sessions_sqlx_store::PostgresStore;
async fn session_layer(db_pool: PgPool) -> Result<SessionManagerLayer<PostgresStore>> {
    let session_store = PostgresStore::new(db_pool);
    
    session_store.migrate().await?;
    
    let deletion_task = tokio::task::spawn(session_store.clone().conti)
    
    let layer = SessionManagerLayer::new(session_store).with_same_site(SameSite::Lax);
    Ok(layer)
}

use tower_http::cors::CorsLayer;
async fn cors_layer() -> Result<CorsLayer> {
    use http::{HeaderValue, Method, header};

    let layer = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST])
        .allow_headers([header::CONTENT_TYPE])
        .allow_credentials(true);
    Ok(layer)
}

/// Shutdown signal, completes when Ctrl + C is pressed or
/// (on Unix) a termination signal is sent.
async fn shutdown_signal() {
    use tokio::signal;

    let ctrl_c = async { signal::ctrl_c().await.unwrap() };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .unwrap()
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}
