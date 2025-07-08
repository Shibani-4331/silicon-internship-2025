use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use axum::Json;

/// Todo operation errors
#[derive(Serialize, Deserialize, ToSchema)]
pub enum TodoError {
    /// Todo already exists conflict.
    #[schema(example = "Todo already exists")]
    Conflict(String),
    /// Todo not found by id.
    #[schema(example = "id = 1")]
    NotFound(String),
    /// Todo operation unauthorized
    #[schema(example = "missing api key")]
    Unauthorized(String),
}

impl IntoResponse for TodoError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            TodoError::Conflict(_) => StatusCode::CONFLICT,
            TodoError::NotFound(_) => StatusCode::NOT_FOUND,
            TodoError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
        };

        (status, Json(self)).into_response()
    }
}

