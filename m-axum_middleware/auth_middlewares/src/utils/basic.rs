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
    let token = token.trim_start_matches("Basic ").trim();
    let decoded_b64 = match BASE64_STANDARD.decode(token) {
        Ok(decoded) => decoded,
        Err(_) => return false,
    };

    let decoded_str = match String::from_utf8(decoded_b64) {
        Ok(s) => s,
        Err(_) => return false,
    };

    let parts: Vec<&str> = decoded_str.splitn(2, ":").collect();
    let username = parts.get(0);
    let password = parts.get(1);

    if let (Some(username),Some(pass)) = (username, password) {
        return *username == expected_username && *pass == expected_password;
    }

    false
}
