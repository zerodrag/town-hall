use std::sync::Arc;

use axum::{Json, extract::State, http::StatusCode};
use sqlx::{Pool, Postgres};

use crate::models::{CreateUserRequest, User};

pub async fn get_user_count(State(pool): State<Arc<Pool<Postgres>>>) -> Result<String, StatusCode> {
    let result = sqlx::query_scalar!("SELECT COUNT(*) FROM users")
        .fetch_one(&*pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(result.to_string())
}

pub async fn create_user(Json(payload): Json<CreateUserRequest>) -> Json<User> {
    let new_user = User {
        id: 1,
        github_id: payload.github_id,
        username: payload.username,
    };
    Json(new_user)
}
