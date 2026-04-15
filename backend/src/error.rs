use axum::response::IntoResponse;
use http::StatusCode;

pub type BackendResult<T> = std::result::Result<T, BackendError>;

#[derive(thiserror::Error, Debug)]
pub enum BackendError {
    #[error("Database Error")]
    SqlxError(#[from] sqlx::Error),
    #[error("Session Error")]
    SessionError(#[from] tower_sessions::session::Error),
    #[error("HTTP Client Error")]
    ReqwestError(#[from] reqwest::Error),
    #[error("OAuth Request Token Error")]
    OAuthRequestTokenError(
        #[from]
        oauth2::RequestTokenError<
            oauth2::HttpClientError<oauth2::reqwest::Error>,
            oauth2::StandardErrorResponse<oauth2::basic::BasicErrorResponseType>,
        >,
    ),
    #[error("JSON (De)Serialization Error")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("Not Found: {0}")]
    NotFound(String),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Bad Request: {0}")]
    BadRequest(String),
    #[error(transparent)]
    ValidationError(#[from] validator::ValidationErrors),
}

impl IntoResponse for BackendError {
    fn into_response(self) -> axum::response::Response {
        let status = match &self {
            BackendError::SqlxError(_)
            | BackendError::SessionError(_)
            | BackendError::ReqwestError(_)
            | BackendError::OAuthRequestTokenError(_)
            | BackendError::SerdeJsonError(_) => {
                tracing::error!(error = ?self, "request failed");
                StatusCode::INTERNAL_SERVER_ERROR
            }
            BackendError::NotFound(_) => StatusCode::NOT_FOUND,
            BackendError::Unauthorized => StatusCode::UNAUTHORIZED,
            BackendError::BadRequest(_) | BackendError::ValidationError(_) => StatusCode::BAD_REQUEST,
        };
        (status, self.to_string()).into_response()
    }
}
