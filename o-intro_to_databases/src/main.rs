use std::sync::{Arc, RwLock};

use axum::{extract::State, routing::{get, post}, Router};

mod book;
mod handler;

use handler::{
    add_book, list_books, get_book, update_book, delete_book, search_books
};

use crate::book::{Book};

// Main function
#[tokio::main]
async fn main() {

    // Standard Router Implementation
    let app = Router::new()
        .route("/all", get(list_books))
        .route("/new", post(add_book))
        .route("/search", get(search_books))
        .route("/{id}", get(get_book)
                        .put(update_book)
                        .delete(delete_book));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    println!("Server running on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

