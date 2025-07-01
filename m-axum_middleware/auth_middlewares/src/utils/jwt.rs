use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use crate::utils::config;

/// A struct representing the claims of a JWT.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub exp: usize,
    pub iat: usize,
    pub sub: String,
    pub user_type: String,
    pub user_role: String
}


/// Creates a JWT token.
///
/// # Arguments
///
/// * `sub` - The subject of the token.
/// * `user_type` - The type of the user.
/// * `user_role` - The role of the user.
///
/// # Returns
///
/// A JWT token string.
pub fn create_jwt_token(sub: String, user_type: String, user_role: String) -> String {
    let issued_at = Utc::now();
    let expiration = issued_at + Duration::days(1); // Token valid for 1 day

    let claims = Claims {
        exp: expiration.timestamp() as usize,
        iat: issued_at.timestamp() as usize,
        sub,
        user_type,
        user_role,
    };

    let header = Header::new(jsonwebtoken::Algorithm::HS256);
    let encoding_key = EncodingKey::from_secret(config::JWT_SECRET_KEY.as_ref());

    encode(&header, &claims, &encoding_key).unwrap()
}

/// Verifies a JWT token.
///
/// # Arguments
///
/// * `token` - The JWT token.
///
/// # Returns
///
/// `true` if the token is valid, `false` otherwise.
pub fn verify_jwt_token(token: &str) -> bool {
    let decoding_key = DecodingKey::from_secret(config::JWT_SECRET_KEY.as_ref());
    let algorithm = Validation::new(jsonwebtoken::Algorithm::HS256);
    decode::<Claims>(token, &decoding_key, &algorithm).is_ok()
}
