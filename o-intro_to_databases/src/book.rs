use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, Pool, Postgres};

/// Represents a Book with an ID, title, and author.
/// Derives necessary traits for debugging, deserialization from CSV/JSON,
/// serialization to CSV/JSON, and cloning.
#[derive(Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
}

pub async fn create_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let connection_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://user:password@localhost/database_name")
        .await
        .expect("Failed to create connection pool");

    Ok(connection_pool)
}

pub async fn get_books_list() -> Result<Option<Vec<Book>>, sqlx::Error> {

    let connection = create_connection().await?;
    let books  = sqlx::query_as::<_, Book>("SELECT id, title, author FROM books")
        .fetch_all(&connection).await?;
    Some(books)
}