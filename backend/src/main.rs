mod handlers;

use axum::routing::*;

use handlers::*;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

fn app() -> axum::Router {
    axum::Router::new()
        .fallback(fallback)
        .route("/", get(hello_world))
        .route("/health", get(health))
}
