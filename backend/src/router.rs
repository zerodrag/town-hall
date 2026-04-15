use anyhow::Result;
use axum::Router;
use axum::routing::get;
use axum::routing::patch;
use axum::routing::post;

use crate::AppState;
use crate::handlers::*;

pub async fn root() -> Result<Router<AppState>> {
    let router = Router::new()
        .fallback(fallback)
        .route("/", get(hello_world))
        .route("/health", get(health))
        .nest("/users", users().await?)
        .nest("/quests", quests().await?)
        .nest("/auth", auth().await?);
    Ok(router)
}

async fn users() -> Result<Router<AppState>> {
    let router = Router::new()
        .route("/{id}", get(user::get))
        .route("/me", get(user::get_me))
        .route("/resolve/{handle}", get(user::resolve_handle_to_id));
    Ok(router)
}

async fn quests() -> Result<Router<AppState>> {
    let router = Router::new()
        .route("/", post(quest::create))
        .route("/{id}", get(quest::get))
        .route("/{id}", patch(quest::update));
    Ok(router)
}

async fn auth() -> Result<Router<AppState>> {
    let router = Router::new()
        .route("/github", get(auth::github_login))
        .route("/github/callback", get(auth::github_callback))
        .route("/logout", get(auth::logout));
    Ok(router)
}
