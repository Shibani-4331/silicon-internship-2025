use axum::{
    extract::{Path, Query, State},
    response::IntoResponse,
    Json,
};
use hyper::StatusCode;
use serde::Deserialize;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter, Set};
use utoipa::IntoParams;
use std::sync::Arc;

use crate::todo::config::TODO_TAG;
use crate::todo::model::TodoError;
use crate::entity::todos::{self, Entity as TodoEntity};

#[derive(Deserialize)]
pub struct Pagination {
    page: Option<u64>,
    limit: Option<u64>,
}

/// List all Todo items
///
/// List all Todo items from the database with pagination.
#[utoipa::path(
    get,
    path = "",
    tag = TODO_TAG,
    params(
        ("page" = Option<u64>, Query, description = "Page number"),
        ("limit" = Option<u64>, Query, description = "Number of items per page")
    ),
    responses(
        (status = 200, description = "List all todos successfully", body = [todos::Model])
    )
)]
pub async fn list_todos(
    State(db): State<Arc<DatabaseConnection>>,
    pagination: Query<Pagination>,
) -> impl IntoResponse{
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10);

    let todos = TodoEntity::find().paginate(&*db, limit).fetch_page(page - 1).await;

    match todos {
        Ok(todos) => (StatusCode::OK, Json(todos)).into_response(),
        Err(e) => TodoError::Conflict(format!("Failed to fetch todos: {}", e)).into_response(),
    }
}

/// Todo search query
#[derive(Deserialize, IntoParams)]
pub struct TodoSearchQuery {
    /// Search by value (case in-sensitive)
    value: String,
}

/// Search Todos by query params.
#[utoipa::path(
    get,
    path = "/search",
    tag = TODO_TAG,
    params(
        TodoSearchQuery
    ),
    responses(
        (status = 200, description = "List matching todos by query", body = [todos::Model])
    )
)]
pub async fn search_todos(
    State(db): State<Arc<DatabaseConnection>>,
    query: Query<TodoSearchQuery>,
) -> impl IntoResponse {
    let search_term = format!("%{}%", query.value.to_lowercase());
    let todos = TodoEntity::find().filter(
        todos::Column::Title.like(&search_term)).all(&*db).await;

    match todos {
        Ok(todos) => (StatusCode::OK, Json(todos)).into_response(),
        Err(e) => TodoError::Conflict(format!("Failed to search todos: {}", e)).into_response(),
    }
}

#[derive(Deserialize, utoipa::ToSchema)]
pub struct CreateTodo {
    pub title: String,
    pub description: Option<String>,
}

/// Create new Todo
#[utoipa::path(
    post,
    path = "",
    tag = TODO_TAG,
    responses(
        (status = 201, description = "Todo item created successfully", body = todos::Model),
        (status = 409, description = "Todo already exists", body = TodoError)
    )
)]
pub async fn create_todo(
    State(db): State<Arc<DatabaseConnection>>,
    Json(todo): Json<CreateTodo>,
) -> impl IntoResponse {
    let new_todo = todos::ActiveModel {
        title: Set(todo.title),
        description: Set(todo.description),
        ..Default::default()
    }.insert(&*db).await;

    match new_todo {
        Ok(todo) => (StatusCode::CREATED, Json(todo)).into_response(),
        Err(e) => TodoError::Conflict(format!("Failed to create todo: {}", e)).into_response(),
    }
}

/// Mark Todo item done by id
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
    State(db): State<Arc<DatabaseConnection>>,
) -> StatusCode {
    let todo = TodoEntity::find_by_id(id).one(&*db).await;
    // Result > Option > Model
    match todo {
        Ok(Some(todo)) => {
            let mut todo = todo.into_active_model();
            todo.completed = Set(Some(true));
            let result = todo.update(&*db).await;
            match result {
                Ok(_) => StatusCode::OK,
                Err(e) => {
                    eprintln!("Failed to mark todo as done: {}", e);
                    StatusCode::INTERNAL_SERVER_ERROR
                }
            }
        },
        _ => {
            eprintln!("Todo with id {} not found", id);
            StatusCode::NOT_FOUND
        }
    }
}

/// Delete Todo item by id
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
    State(db): State<Arc<DatabaseConnection>>,
) -> impl IntoResponse {
    let result = TodoEntity::delete_by_id(id).exec(&*db).await;

    match result {
        Ok(result) if result.rows_affected > 0 => StatusCode::OK.into_response(),
        _ => (
            StatusCode::NOT_FOUND,
            Json(TodoError::NotFound(format!("id = {id}"))),
        )
            .into_response(),
    }
}

