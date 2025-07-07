use axum::{middleware, routing::*};
use utoipa_axum::{router::OpenApiRouter};
use sqlx::PgPool;

use super::{handler, middleware::auth};

pub fn router(pool: PgPool) -> OpenApiRouter {
    OpenApiRouter::new()
        .route("/", post(handler::create_todo))
        .route("/", get(handler::list_todos))
        .route("/search", get(handler::search_todos))
        .route("/{id}", put(handler::mark_done))
        .route("/{id}", delete(handler::delete_todo))
        .route_layer(middleware::from_fn(auth))
        .with_state(pool)
}