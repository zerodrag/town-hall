use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};
use color_eyre::eyre::Result;
use std::sync::Arc;

use crate::{db, handlers::*};

pub async fn api_router() -> Result<Router> {
    let shared_state = Arc::new(db::init().await?);
    let router = Router::new()
        .route("/create-user", post(create_user))
        .route("/user-count", get(get_user_count))
        .route("/health", get(|| async { StatusCode::OK }))
        .with_state(shared_state);
    Ok(router)
}
