use std::sync::{Arc, RwLock};

use axum::{extract::State, routing::{get, post, put, delete}, Router};

mod book;
mod handler;

use book::create_connection;

use crate::book::{Book};

// Main function
#[tokio::main]
async fn main() {

    let db_connection = create_connection().await.expect("Failed to create connection");

    sqlx::query(
        r#"
            CREATE TABLE IF NOT EXISTS books (  
                id SERIAL PRIMARY KEY,
                title VARCHAR(255) NOT NULL,
                author VARCHAR(255) NOT NULL
            );
        "#
    ).execute(&db_connection).await.expect("Failed to create table");

    // Standard Router Implementation
    let app = Router::new()
        .route("/all", get(handler::list_books_handler))
        .route("/new", post(handler::add_book_handler))
        .route("/search", get(handler::search_books_handler))
        .route("/{id}", get(handler::get_book_handler)
                        .put(handler::update_book_handler)
                        .delete(handler::delete_book_handler))
        .with_state(db_connection);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to address");

    println!("Server running on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

