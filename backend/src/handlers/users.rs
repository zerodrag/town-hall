use axum::{Json, extract::State};
use sqlx::PgPool;

use crate::{
    db,
    error::BackendError,
    models::{CreateUserRequest, User},
};

pub async fn get_user_count(State(pool): State<PgPool>) -> Result<String, BackendError> {
    db::get_user_count(&pool).await.map(|n| n.to_string())
}

pub async fn create_user(Json(payload): Json<CreateUserRequest>) -> Json<User> {
    let new_user = User {
        id: 1,
        github_id: payload.github_id,
        username: payload.username,
    };
    Json(new_user)
}
