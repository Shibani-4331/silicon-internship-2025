use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
}
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::env;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

#[derive(Debug)]
pub struct AuthUser {
    pub u_id: String,
    pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, (StatusCode, String)> {
       let auth_header = parts.headers().get("Authorization")
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?
            .to_str().map_err(|_| (StatusCode::BAD_REQUEST, "Invalid Authorization header"
            .to_string()))?;

        if !auth_header.starts_with("Bearer ") {
            return Err((StatusCode::UNAUTHORIZED, "Invalid Authorization header"
            .to_string()));
        }

       let token = auth_header.trim_start_matches("Bearer ").trim();
        let decoded = decode::<Claims>(
            token,
            &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
            &Validation::default()
        ).map_err(|err| {
            (StatusCode::UNAUTHORIZED, format!("Invalid token: {}", err))
        })?;

        Ok(AuthUser {
            u_id: decoded.claims.sub,
            role: decoded.claims.role,
        })
    }
}