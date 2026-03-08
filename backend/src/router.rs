use axum::{
    Router,
    http::StatusCode,
    routing::{get, post},
};
use color_eyre::eyre::Result;
use sqlx::PgPool;

use crate::handlers::*;

pub async fn api_router(pool: PgPool) -> Result<Router> {
    let router = Router::new()
        .route("/create-user", post(create_user))
        .route("/user-count", get(get_user_count))
        .route("/health", get(|| async { StatusCode::OK }))
        .with_state(pool);
    Ok(router)
}
