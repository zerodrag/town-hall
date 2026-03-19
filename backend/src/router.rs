use anyhow::Result;
use axum::Router;
use axum::routing::get;

use crate::AppState;
use crate::handlers::*;

pub async fn root() -> Result<Router<AppState>> {
    let router = Router::new()
        .fallback(fallback)
        .nest("/api", api().await?)
        .nest("/auth", auth().await?);
    Ok(router)
}

async fn api() -> Result<Router<AppState>> {
    let router = Router::new()
        .route("/", get(hello_world))
        .route("/health", get(health))
        .route("/users/{id}", get(get_user_from_url))
        .route("/users/me", get(get_user_me));
    Ok(router)
}

async fn auth() -> Result<Router<AppState>> {
    let router = Router::new()
        .route("/github", get(github_login))
        .route("/github/callback", get(github_callback))
        .route("/logout", get(logout));
    Ok(router)
}
