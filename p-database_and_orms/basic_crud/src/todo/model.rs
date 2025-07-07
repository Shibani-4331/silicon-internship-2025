use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use sqlx::FromRow;

/// Item to do.
#[derive(Serialize, Deserialize, ToSchema, Clone, FromRow)]
pub struct Todo {
    pub id: i32,
    #[schema(example = "My first todo")]
    pub title: String,
    #[schema(example = "Buy groceries")]
    pub description: String,
    pub completed: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateTodo {
    pub title: String,
    pub description: String,
}

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

