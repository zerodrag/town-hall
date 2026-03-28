use http::StatusCode;
use tower_sessions::Session;

/// Simple wrapper type. Requires T to implement IntoResponse to be used as handler response type.
pub type SimpResp<T> = Result<T, (StatusCode, &'static str)>;

pub async fn resolve_current_user_id(session: &Session) -> SimpResp<i64> {
    let Ok(user_id): Result<Option<i64>, _> = session.get("user_id").await else {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, "Session error"));
    };

    match user_id {
        Some(id) => Ok(id),
        None => Err((StatusCode::UNAUTHORIZED, "Not logged in")),
    }
}
