use anyhow::Result;
use axum::Router;
use axum::routing::get;

use crate::AppState;
use crate::handlers::*;

pub async fn root() -> Result<Router<AppState>> {
    let router = Router::new()
        .fallback(fallback)
        .route("/", get(hello_world))
        .route("/health", get(health))
        .nest("/users", users().await?)
        .nest("/auth", auth().await?);
    Ok(router)
}

async fn users() -> Result<Router<AppState>> {
    let router = Router::new()
        .route("/{id}", get(get_user_from_url))
        .route("/me", get(get_user_me))
        .route("/resolve/{handle}", get(resolve_handle_to_id));
    Ok(router)
}

async fn auth() -> Result<Router<AppState>> {
    let router = Router::new()
        .route("/github", get(github_login))
        .route("/github/callback", get(github_callback))
        .route("/logout", get(logout));
    Ok(router)
}
