pub mod handlers;
mod router;
mod state;

use anyhow::Result;

pub use crate::state::AppState;

use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long, env = "BACKEND_HOST", default_value = "http://localhost")]
    backend_host: String,

    #[arg(long, env = "BACKEND_PORT", default_value = "3000")]
    backend_port: u16,

    #[arg(long, env = "FRONTEND_URL", default_value = "http://localhost:5173")]
    frontend_url: String,

    #[arg(long, env = "DATABASE_URL")]
    database_url: String,

    #[arg(long, env = "GITHUB_CLIENT_ID")]
    github_client_id: String,

    #[arg(long, env = "GITHUB_CLIENT_SECRET")]
    github_client_secret: String,

    #[arg(
        long,
        env = "GEN_TS_TYPES_PATH",
        default_value = "../frontend/src/lib/backend-types.ts"
    )]
    gen_ts_types_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?; // Export .env
    tracing_subscriber::fmt::init(); // Print tracing::_
    let args = Args::parse(); // Parse CLI args

    gen_types(args.gen_ts_types_path).await?;

    let state = AppState::new(
        args.database_url,
        args.frontend_url,
        format!("{}:{}", args.backend_host, args.backend_port),
        args.github_client_id,
        args.github_client_secret,
    )
    .await?;
    let session_layer = session_layer(state.db_pool.clone()).await?;
    let cors_layer = cors_layer().await?;

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", args.backend_port)).await?;
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

async fn gen_types(path: String) -> Result<()> {
    use specta_typescript::Typescript;

    let types = specta::collect();
    let types_str = Typescript::default()
        .bigint(specta_typescript::BigIntExportBehavior::String)
        .export(&types)?;
    tokio::fs::write(path, types_str).await?;
    Ok(())
}

use sqlx::PgPool;
use tower_sessions::{SessionManagerLayer, cookie::SameSite};
use tower_sessions_sqlx_store::PostgresStore;
async fn session_layer(db_pool: PgPool) -> Result<SessionManagerLayer<PostgresStore>> {
    let session_store = PostgresStore::new(db_pool);

    session_store.migrate().await?;

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
