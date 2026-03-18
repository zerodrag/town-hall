mod user;
pub use user::*;
mod quest;
pub use quest::*;
mod auth;
pub use auth::*;

use axum::{
    http::{StatusCode, Uri},
    response::IntoResponse,
};
use tokio::signal;

pub async fn hello_world() -> impl IntoResponse {
    "Hello, World!"
}

pub async fn health() -> impl IntoResponse {
    (
        StatusCode::OK,
        "Welcome to our Town Hall health center!\n\
        We heal your vibe coded ego back to perfect health!\n\
        Shall we process your projects?\n\
        ...\n\
        Ok. We'll need your repo.\n\
        ...\n\
        Di-di-di-di-ding! ♪\n\
        ...\n\
        Thank you! Your codebases are fighting fit!\n\
        We hope to see you again!",
    )
}

pub async fn fallback(uri: Uri) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, uri.to_string())
}

/// Shutdown signal, completes when Ctrl + C is pressed or
/// (on Unix) a termination signal is sent.
pub async fn shutdown_signal() {
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
