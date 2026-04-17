use axum::{
    Json,
    extract::{FromRequest, rejection::JsonRejection},
};
use serde::de::DeserializeOwned;
use tower_sessions::Session;
use validator::Validate;

use crate::{BackendError, BackendResult};

/// Returns Ok(id) if logged in and Err((StatusCode::UNAUTHORIZED)) if not
pub async fn resolve_me_id(session: &Session) -> BackendResult<i64> {
    let id_result = session.get("user_id").await?;
    match id_result {
        Some(val) => Ok(val),
        None => Err(BackendError::Unauthorized),
    }
}

pub trait Normalize {
    fn normalize(&mut self);
}

/// Extractor that normalizes then validates
pub struct NormValidJson<T>(pub T);
impl<T, S> FromRequest<S> for NormValidJson<T>
where
    T: DeserializeOwned + Validate + Normalize,
    S: Send + Sync,
{
    type Rejection = BackendError;

    async fn from_request(req: axum::extract::Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(mut value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|e: JsonRejection| BackendError::BadRequest(e.to_string()))?;
        value.normalize();
        value.validate().map_err(|e| BackendError::ValidationError(e))?;

        Ok(NormValidJson(value))
    }
}

pub fn trim_whitespace(s: &str) -> String {
    s.split_whitespace().collect::<Vec<&str>>().join(" ")
}
