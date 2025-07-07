use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
    Json,
};

use super::model::TodoError;
use crate::todo::config::API_KEY;

pub async fn auth(req: Request, next: Next) -> Result<Response, impl IntoResponse> {
    let headers = req.headers();
    println!("Headers: {:?}", headers);
    
    let api_key = req
        .headers()
        .get("X-Api-Key")
        .and_then(|header| header.to_str().ok());

    println!("API Key: {:?}", api_key);

    match api_key {
        Some(key) if key == API_KEY => Ok(next.run(req).await),
        Some(_) => {
            let error_response = (
                StatusCode::UNAUTHORIZED,
                Json(TodoError::Unauthorized("Incorrect API Key".to_string())),
            );
            Err(error_response)
        }
        None => {
            let error_response = (
                StatusCode::UNAUTHORIZED,
                Json(TodoError::Unauthorized("Missing API Key".to_string())),
            );
            Err(error_response)
        }
    }
}
