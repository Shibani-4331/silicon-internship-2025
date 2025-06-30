use base64::prelude::*;

/// Creates a Basic Auth token from a username and password.
///
/// # Arguments
///
/// * `username` - The username.
/// * `password` - The password.
///
/// # Returns
///
/// A Basic Auth token string.
pub fn create_basic_auth_token(username: &str, password: &str) -> String {
    let data_to_encode = format!("{}:{}", username, password);
    let token = BASE64_STANDARD.encode(data_to_encode);
    format!("Basic {}", token)
}

/// Verifies a Basic Auth token.
///
/// # Arguments
///
/// * `token` - The Basic Auth token.
/// * `expected_username` - The expected username.
/// * `expected_password` - The expected password.
///
/// # Returns
///
/// `true` if the token is valid, `false` otherwise.
pub fn verify_basic_auth_token(
    token: &str,
    expected_username: &str,
    expected_password: &str,
) -> bool {
    todo!("Verify Basic Token")
}
