use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, Pool, Postgres};

/// Represents a Book with an ID, title, and author.
/// Derives necessary traits for debugging, deserialization from CSV/JSON,
/// serialization to CSV/JSON, and cloning.
#[derive(Debug, Deserialize, Serialize, Clone, FromRow)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub author: String,
}

pub async fn create_connection() -> Result<Pool<Postgres>, sqlx::Error> {
    let connection_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://myuser:mypassword@192.168.29.184:5432/mydatabase")
        .await
        .expect("Failed to create connection pool");

    Ok(connection_pool)
}

pub async fn get_books_list(connection: &Pool<Postgres>) -> Result<Option<Vec<Book>>, sqlx::Error> {

    // let connection = create_connection().await?;
    let books  = sqlx::query_as::<_, Book>
        ("SELECT id, title, author FROM books")
        .fetch_all(connection).await?;
    Ok(Some(books))
}

pub async fn add_book(connection: &Pool<Postgres>, title: String, author: String) -> Result<(), sqlx::Error> {
    // let connection = create_connection().await?;
    
    sqlx::query("INSERT INTO books (title, author) VALUES ($1, $2)")
        .bind(&title)
        .bind(&author)
        .execute(connection)
        .await?;

    Ok(())
}

pub async fn get_book_by_id(connection: &Pool<Postgres>, id: i32) -> Result<Option<Book>, sqlx::Error> {
    // let connection = create_connection().await?;
    
    let book = sqlx::query_as::<_, Book>("SELECT id, title, author FROM books WHERE id = $1")
        .bind(id)
        .fetch_optional(connection)
        .await?;

    Ok(book)
}

pub async fn update_book(connection: &Pool<Postgres>, book: Book) -> Result<(), sqlx::Error> {
    // let connection = create_connection().await?;
    
    sqlx::query("UPDATE books SET title = $1, author = $2 WHERE id = $3")
        .bind(&book.title)
        .bind(&book.author)
        .bind(&book.id)
        .execute(connection)
        .await?;

    Ok(())
}

pub async fn delete_book(connection: &Pool<Postgres>, id: i32) -> Result<(), sqlx::Error> {
    
    sqlx::query("DELETE FROM books WHERE id = $1")
        .bind(id)
        .execute(connection)
        .await?;

    Ok(())
}

pub async fn search_books(connection: &Pool<Postgres>, title: Option<String>) -> Result<Vec<Book>, sqlx::Error> {


    // let search_params = vec!["title", "author"];

    // for param in search_params {
    //     if let Some(value) = title.as_ref() {
    //         if !value.is_empty() {
    //             query.push_str(&format!(" WHERE {} ILIKE $1", param));
    //             params.push(value.clone());
    //         }
    //     }
    //     query.push_str(&format!(" AND {} ILIKE $1", param));
    // }

    if title.is_none() {
        return Ok(vec![]);
    }

    let mut query = String::from("SELECT id, title, author FROM books");
    let mut title_value = title.clone().unwrap_or_default();
    if title.is_some() {
        query.push_str(" WHERE title ILIKE $1");
        title_value = format!("%{}%", title_value.to_lowercase());
    }

    // "" == []

    let books = sqlx::query_as::<_, Book>(&query)
        .bind(title_value)
        .fetch_all(connection)
        .await?;

    Ok(books)
}