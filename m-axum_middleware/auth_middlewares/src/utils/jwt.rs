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
pub fn create_jwt_token(sub: String, user_type: String) -> String {
    todo!("Create JWT Token")
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
    todo!("Verify JWT Token")
}
