use axum::{
    routing::{post, get},
    extract::{State},
    Router,
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, Set, ActiveModelTrait};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::types::chrono::Utc;
use sea_orm::prelude::Uuid;
use crate::entity::users::ActiveModel;
use crate::{app_state::AppState, entity::users};

pub use users::Entity as UserEntity;


#[derive(Deserialize)]
struct CreateUserInput {
    email: String,
    name: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: i32,
    email: String,
    name: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/users", post(create_user))
        .route("/users", get(get_users))
}

 pub async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<CreateUserInput>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = users::ActiveModel {
        id: Set(uuid::Uuid::new_v4().to_string()),
        email: Set(input.email),
        name: Set(input.name),
        created_at:Set(chrono::Utc::now().into())
    };

    let db = &state.db;
    let res = ActiveModel::insert(users, db).await.map_err(|e| {
        eprintln!("Failed to create user: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user".into())
    })?;

    Ok(Json(UserResponse {
        id: res.id,
        email: res.email,
        name: res.name,
    }))
}

pub async fn get_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, (StatusCode, String)> {
    let db = &state.db;
    let users = users::find()
        .all(db)
        .await
        .map_err(|e| {
            eprintln!("Failed to retrieve users: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to retrieve users".into())
        })?;

     let response = users
        .into_iter()
        .map(|user| UserResponse {
            id: user.id.to_string(),
            email: user.email,
            name: user.name,
        })
        .collect();

    Ok(Json(response))
}