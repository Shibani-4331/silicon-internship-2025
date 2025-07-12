use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Validation, Header, encode};
use serde::{Deserialize, Serialize};
use once_cell::sync::Lazy;
use std::env;
use async_trait::async_trait;
use chrono::{Duration, Utc};


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

// #[async_trait]
impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get("Authorization")
            .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header".to_string()))?
            .to_str().map_err(|_| {
                (StatusCode::BAD_REQUEST, "Invalid Authorization header".to_string())
            })?;

        if !auth_header.starts_with("Bearer ") {
            return Err((StatusCode::UNAUTHORIZED, "Invalid Authorization header".to_string()));
        }

        let token = auth_header.trim_start_matches("Bearer ").trim();

        let secret = env::var("JWT_SECRET")
            .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "JWT_SECRET not set".to_string()))?;
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());

        let decoded = decode::<Claims>(
            token,
            &decoding_key,
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

pub fn generate_jwt(user_id: &str, role: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        role: role.to_owned(),
        exp: expiration,
    };

    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let encoding_key = EncodingKey::from_secret(jwt_secret.as_bytes());


    encode(
        &Header::default(),
        &claims,
        &encoding_key,
    )
    .expect("Token creation failed")
}


//verify
pub fn require_role(user: &AuthUser, required_role: &str) -> Result<(), (StatusCode, String)> {
    if user.role != required_role {
        Err((StatusCode::FORBIDDEN, "Insufficient permissions".into()))
    } else {
        Ok(())
    }
}   