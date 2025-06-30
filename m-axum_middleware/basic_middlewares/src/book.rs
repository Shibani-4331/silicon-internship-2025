use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    body::Body,
    Json
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Represents a Book with an ID, title, and author.
/// Derives necessary traits for debugging, deserialization from CSV/JSON,
/// serialization to CSV/JSON, and cloning.
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
}

/// Create Error type for book operations.
#[derive(Debug, Error)]
pub enum BookError {
    #[error("Book not found: {0}")]
    NotFound(String),
    #[error("Invalid data: {0}")]
    InvalidData(String),
    #[error("Server error: {0}")]
    ServerError(String)
}

/// Implement IntoResponse for BookError to convert it into an HTTP response.
impl IntoResponse for BookError {
    fn into_response(self) -> Response {
        match self {
            BookError::NotFound(msg) => (StatusCode::NOT_FOUND, msg).into_response(),
            BookError::InvalidData(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            BookError::ServerError(msg) => {
                // Log message for debugging purposes
                tracing::trace!("Server error: {}", msg);
                // Return a generic internal server error response
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            },
        }
    }
}

/// Loads book data from the CSV file.
///
/// Reads the CSV file specified by `CSV_FILE`, deserializes each row into a `Book` struct,
/// and returns a vector of `Book`s.
///
/// # Returns
///
/// `Result<Vec<Book>, csv::Error>` - A `Result` containing the vector of books on success,
/// or a `csv::Error` if reading or deserializing fails.
pub fn load_books_from_csv() -> Result<Vec<Book>, BookError> {
    let mut reader = csv::Reader::from_path("assets/books.csv").map_err(|err| {
        tracing::error!("Failed to open CSV file for reading: {}", err);
        BookError::ServerError("Failed to open CSV file for reading".to_string())
    })?;
    let mut books = Vec::new();

    for result in reader.deserialize() {
        let book: Book = result.map_err(|err| {
            // Log error
            tracing::error!("Failed to deserialize book: {}", err);
            // Return an invalid data error if deserialization fails
            BookError::InvalidData("Failed to deserialize book".to_string())
        })?;
        books.push(book);
    }

    Ok(books)
}

/// Saves the current book data to the CSV file.
///
/// Writes the provided slice of `Book`s to the CSV file, overwriting its previous content.
///
/// # Arguments
///
/// * `books` - A slice of `Book`s to be saved.
///
/// # Returns
///
/// `Result<(), csv::Error>` - An `Ok(())` on success, or a `csv::Error` if writing or serializing fails.
pub fn save_books_to_csv(books: &[Book]) -> Result<(), BookError> {
    let mut writer = csv::Writer::from_path("assets/books.csv").map_err(|err| {
        tracing::error!("Failed to open CSV file for writing: {}", err);
        BookError::ServerError("Failed to open CSV file for writing".to_string())
    })?;

    for book in books {
        writer.serialize(book).map_err(|err| {
            tracing::error!("Failed to serialize book: {}", err);
            BookError::InvalidData("Failed to serialize book".to_string())
        })?;
    }

    writer.flush().map_err(|err| {
        tracing::error!("Failed to flush CSV writer: {}", err);
        BookError::ServerError("Failed to flush CSV writer".to_string())
    })?;

    Ok(())
}