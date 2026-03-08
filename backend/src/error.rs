use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum BackendError {
    Sqlx(sqlx::Error),
}

impl IntoResponse for BackendError {
    fn into_response(self) -> Response {
        match self {
            Self::Sqlx(ref e) => {
                // TODO: Log error
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal database error occurred",
                )
            }
            .into_response(),
        }
    }
}

impl From<sqlx::Error> for BackendError {
    fn from(err: sqlx::Error) -> Self {
        Self::Sqlx(err)
    }
}
