use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Item to do.
#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct Todo {
    pub id: i32,
    #[schema(example = "Buy groceries")]
    pub value: String,
    pub done: bool,
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
