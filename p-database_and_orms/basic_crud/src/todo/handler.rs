use super::model::{Todo, CreateTodo};
use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use serde::Deserialize;
use sqlx::PgPool;
use utoipa::IntoParams;

use crate::todo::model::TodoError;
use crate::todo::config::TODO_TAG;

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<i64>,
    limit: Option<i64>,
}

/// List all Todo items
///
/// List all Todo items from the database with pagination.
#[utoipa::path(
    get,
    path = "",
    tag = TODO_TAG,
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("limit" = Option<i64>, Query, description = "Number of items per page")
    ),
    responses(
        (status = 200, description = "List all todos successfully", body = [Todo])
    )
)]
pub async fn list_todos(
    State(pool): State<PgPool>,
    pagination: Query<Pagination>,
) -> Result<Json<Vec<Todo>>, impl IntoResponse> {
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);
    let offset = (page - 1) * limit;

    let todos = sqlx::query_as::<_, Todo>("SELECT * FROM todos ORDER BY id LIMIT $1 OFFSET $2")
        .bind(limit)
        .bind(offset)
        .fetch_all(&pool)
        .await;

    match todos {
        Ok(todos) => Ok(Json(todos)),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch todos").into_response()),
    }
}

/// Todo search query
#[derive(Deserialize, IntoParams)]
pub struct TodoSearchQuery {
    /// Search by value. Search is incase sensitive.
    value: String,
}

/// Search Todos by query params.
///
/// Search `Todo`s by query params and return matching `Todo`s.
#[utoipa::path(
    get,
    path = "/search",
    tag = TODO_TAG,
    params(
        TodoSearchQuery
    ),
    responses(
        (status = 200, description = "List matching todos by query", body = [Todo])
    )
)]
pub async fn search_todos(
    State(pool): State<PgPool>,
    query: Query<TodoSearchQuery>,
) -> Result<Json<Vec<Todo>>, impl IntoResponse> {
    let search_term = format!("%{}%", query.value.to_lowercase());
    let todos = sqlx::query_as::<_, Todo>(
        "SELECT * FROM todos WHERE lower(title) LIKE $1 OR lower(description) LIKE $1",
    )
    .bind(search_term)
    .fetch_all(&pool)
    .await;

    match todos {
        Ok(todos) => Ok(Json(todos)),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to search todos").into_response()),
    }
}



/// Create new Todo
///
/// Tries to create a new Todo item to in-memory storage or fails with 409 conflict if already exists.
#[utoipa::path(
    post,
    path = "",
    tag = TODO_TAG,
    responses(
        (status = 201, description = "Todo item created successfully", body = Todo),
        (status = 409, description = "Todo already exists", body = TodoError)
    )
)]
pub async fn create_todo(
    State(pool): State<PgPool>,
    Json(todo): Json<CreateTodo>,
) -> impl IntoResponse {
    let new_todo = sqlx::query_as::<_, Todo>(
        "INSERT INTO todos (title, description) VALUES ($1, $2) RETURNING *",
    )
    .bind(todo.title)
    .bind(todo.description)
    .fetch_one(&pool)
    .await;

    match new_todo {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(TodoError::Conflict("Failed to create todo".to_string())),
        )
            .into_response(),
    }
}

/// Mark Todo item done by id
///
/// Mark Todo item done by given id. Return only status 200 on success or 404 if Todo is not found.
#[utoipa::path(
    put,
    path = "/{id}",
    tag = TODO_TAG,
    responses(
        (status = 200, description = "Todo marked done successfully"),
        (status = 404, description = "Todo not found")
    ),
    params(
        ("id" = i32, Path, description = "Todo database id")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn mark_done(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> StatusCode {
    let result = sqlx::query("UPDATE todos SET completed = true WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(result) if result.rows_affected() > 0 => StatusCode::OK,
        _ => StatusCode::NOT_FOUND,
    }
}

/// Delete Todo item by id
///
/// Delete Todo item from in-memory storage by id. Returns either 200 success of 404 with TodoError if Todo is not found.
#[utoipa::path(
    delete,
    path = "/{id}",
    tag = TODO_TAG,
    responses(
        (status = 200, description = "Todo marked done successfully"),
        (status = 401, description = "Unauthorized to delete Todo", body = TodoError, example = json!(TodoError::Unauthorized(String::from("missing api key")))),
        (status = 404, description = "Todo not found", body = TodoError, example = json!(TodoError::NotFound(String::from("id = 1"))))
    ),
    params(
        ("id" = i32, Path, description = "Todo database id")
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn delete_todo(
    Path(id): Path<i32>,
    State(pool): State<PgPool>,
) -> impl IntoResponse {
    let result = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(result) if result.rows_affected() > 0 => StatusCode::OK.into_response(),
        _ => (
            StatusCode::NOT_FOUND,
            Json(TodoError::NotFound(format!("id = {id}"))),
        )
            .into_response(),
    }
}
