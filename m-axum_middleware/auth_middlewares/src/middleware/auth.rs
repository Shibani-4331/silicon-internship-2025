use std::collections::HashMap;

use crate::utils::{basic, config, jwt};
use axum::{
    body::Body,
    http::{HeaderValue, Request, StatusCode},
    middleware::Next,
    response::Response,
};
use tower_http::auth;

/// A middleware for JWT authentication.
///
/// This middleware checks for the `authorization` header and validates the
/// JWT token.
///
/// # Arguments
///
/// * `req` - The incoming request.
/// * `next` - The next middleware in the chain.
///
/// # Returns
///
/// * `Ok(Response)` if the token is valid.
/// * `Err(StatusCode::UNAUTHORIZED)` if the token is missing or invalid.
pub async fn jwt_auth(req: Request<Body>, next: Next) -> Result<Response, (StatusCode, String)> {

    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|value| value.to_str().ok());

    // // Handle case sensitive header retrieval
    // let lower_cased_headers: HashMap<String, HeaderValue> = req
    //     .headers()
    //     .iter()
    //     .filter_map(|(k, v)| {
    //         if k.as_str().eq_ignore_ascii_case("authorization") {
    //             Some((k.as_str().to_string(), v.clone()))
    //         } else {
    //             None
    //         }
    //     })
    //     .collect();

    // tracing::debug!("Lower cased headers: {:?}", lower_cased_headers);

    // let auth_header = lower_cased_headers
    //     .get("authorization")
    //     .and_then(|value| value.to_str().ok());

    // tracing::debug!("Authorization header: {:?}", auth_header);

    match auth_header {
        Some(header) if header.starts_with("Bearer ") => {
            let token = header.trim_start_matches("Bearer ");
            if jwt::verify_jwt_token(token) {
                Ok(next.run(req).await)
            } else {
                Err((StatusCode::UNAUTHORIZED, "Invalid JWT token".to_string()))
            }
        }
        _ => Err((StatusCode::UNAUTHORIZED, "Missing authorization header".to_string())),
    }
}


/// A middleware for validating an API key.
///
/// This middleware checks for the presence of the `x-api-key` header and
/// validates its value against the `API_KEY` constant.
///
/// # Arguments
///
/// * `req` - The incoming request.
/// * `next` - The next middleware in the chain.
///
/// # Returns
///
/// * `Ok(Response)` if the API key is valid.
/// * `Err(StatusCode::UNAUTHORIZED)` if the API key is missing or invalid.
pub async fn api_key_auth(req: Request<Body>, next: Next) -> Result<Response, (StatusCode, String)> {
    let api_key = req
        .headers()
        .get("x-api-key")
        .and_then(|value| value.to_str().ok());

    match api_key {
        Some(key) if key == config::API_KEY => Ok(next.run(req).await),
        _ => Err((StatusCode::UNAUTHORIZED, "Invalid API key".to_string())),
    }
}

/// A middleware for basic authentication.
///
/// This middleware checks for the `authorization` header and validates the
/// credentials using basic authentication.
///
/// # Arguments
///
/// * `req` - The incoming request.
/// * `next` - The next middleware in the chain.
///
/// # Returns
///
/// * `Ok(Response)` if the credentials are valid.
/// * `Err(StatusCode::UNAUTHORIZED)` if the credentials are missing or invalid.
pub async fn basic_auth(req: Request<Body>, next: Next) -> Result<Response, (StatusCode, String)> {
    let auth_header = req
        .headers()
        .get("authorization")
        .and_then(|value| value.to_str().ok());

    match auth_header {
        Some(header) => {
            if basic::verify_basic_auth_token(header, config::BASIC_AUTH_USERNAME, config::BASIC_AUTH_PASSWORD) {
                Ok(next.run(req).await)
            } else {
                Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))
            }
        }
        _ => Err((StatusCode::UNAUTHORIZED, "Missing authorization header".to_string())),
    }
}
