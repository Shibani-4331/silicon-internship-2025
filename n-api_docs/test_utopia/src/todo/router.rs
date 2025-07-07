use std::sync::Arc;

use axum::{middleware, routing::*};
use utoipa_axum::{router::OpenApiRouter};

use super::{handler, middleware::auth, store::Store};

pub fn router() -> OpenApiRouter {
    let store = Arc::new(Store::default());

    OpenApiRouter::new()
        .route("/", post(handler::create_todo))
        .route("/", get(handler::list_todos))
        .route("/search", get(handler::search_todos))
        .route("/{id}", put(handler::mark_done))
        .route("/{id}", delete(handler::delete_todo))
        .route_layer(middleware::from_fn(auth))
        .with_state(store)
}