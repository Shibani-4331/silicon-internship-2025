use std::sync::{Arc, RwLock};
use axum::{body::Body, extract::{Request, State}, routing::{get, post}, Router
};

use tracing_subscriber::{filter, fmt, EnvFilter};
use tracing_subscriber::prelude::*;

mod book;
mod handler;
mod middleware_util;

use handler::{
    add_book, list_books, get_book, update_book, delete_book, search_books
};


use crate::book::{Book, load_books_from_csv, save_books_to_csv};
type AppState = State<Arc<RwLock<Vec<Book>>>>;

// Main function
#[tokio::main]
async fn main() {
    
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("trace")) // default to `trace`
        .with_target(false)                       // optional: hide module paths
        .with_level(true)                         // show log level in output
        .init();

    // Separate tracing subscriber setup with custom layers
    // let fmt_layer = fmt::layer()
    //     .with_level(true)
    //     .with_target(false);

    // let filter_layer = EnvFilter::try_from_default_env()
    //     .or_else(|_| EnvFilter::try_new("trace"))
    //     .unwrap();

    // tracing_subscriber::registry()
    //     .with(filter_layer)
    //     .with(fmt_layer)
    //     .init();

    let books = load_books_from_csv().unwrap_or_else(|_| {
        eprintln!("Failed to load books, starting with an empty list.");
        Vec::new()
    });

    let shared_books = Arc::new(RwLock::new(books));

    
    // Standard Router Implementation
    let app = Router::new()
        .route("/all", get(list_books))
        .route("/new", post(add_book))
        .route("/search", get(search_books))
        .route("/{id}", get(get_book)
                        .put(update_book)
                        .delete(delete_book))
        // .layer(axum::middleware::from_fn(middleware_util::log_request_raw))
        .with_state(shared_books);

    let app = middleware_util::log_request_tracing(app);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    println!("Server running on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

