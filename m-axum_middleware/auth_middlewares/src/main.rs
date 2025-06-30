use axum::{
    body::Body, http::{Request, StatusCode}, middleware::{self as axum_middleware}, response::Response, routing::{get, post}, Form, Json, Router
};
use serde::Deserialize;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{EnvFilter};

use crate::utils::config;

mod middleware;
mod utils;

/// The main entry point of the application.
#[tokio::main]
async fn main() {
    // Initialize the tracing subscriber to log events.
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("debug"))
        .with_target(false) // optional: hide module paths
        .with_level(true) // show log level in output
        .init();

    let basic_auth_router: Router = Router::new()
        .route("/basic_auth", get(basic_auth_handler))
        .route_layer(axum_middleware::from_fn(middleware::auth::basic_auth));

    let api_key_router: Router = Router::new()
        .route("/api_key", get(api_key_handler))
        .route_layer(axum_middleware::from_fn(middleware::auth::api_key_auth));

    let jwt_router: Router = Router::new()
        .route("/jwt", get(jwt_handler))
        .route_layer(axum_middleware::from_fn(middleware::auth::jwt_auth));

    let user_router: Router = Router::new()
        .route("/login", post(user_login_handler))
        .route_layer(axum_middleware::from_fn(middleware::auth::jwt_auth));

    let admin_router: Router = Router::new()
        .route("/gen_basic", post(gen_basic_auth_handler))
        .route("/gen_jwt", post(gen_jwt_handler));

    let app = Router::new()
        .route("/", get(root_handler))
        .merge(basic_auth_router)
        .merge(api_key_router)
        .merge(jwt_router)
        .nest("/user", user_router)
        .nest("/admin", admin_router)
        // Apply a `TraceLayer` to all routes for logging requests and responses.
        .layer(
            TraceLayer::new_for_http()
                // Create a new span for each request.
                // Span means a logical unit of work in tracing which groups related events together.
                .make_span_with(|_request: &Request<Body>| {
                    tracing::info_span!("http-request")
                })
                // Log the request details when a request is received.
                .on_request(|request: &Request<Body>, _span: &tracing::Span| {
                    
                    // Check log levels priority order
                    tracing::trace!("on_request: {:?}", request);
                    tracing::debug!("on_request: {:?}", request);
                    tracing::warn!("on_request: {:?}", request);
                    tracing::error!("on_request: {:?}", request);
                    tracing::info!("on_request: {:?}", request);

                })
                // Log the response details when a response is sent.
                .on_response(|response: &Response, _latency: std::time::Duration, _span: &tracing::Span| {
                    tracing::info!("on_response: {:?}", response)
                })
        );

        // Enable CORS
        let app = app.layer(CorsLayer::permissive());

    // Define the address to listen on.
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    // Bind a TCP listener to the address.
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    // Start the Axum server.
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

/// A simple handler that returns a static string.
async fn root_handler() -> &'static str {
    "Hello, World!"
}

/// A handler that is only accessible with successful basic authentication.
async fn basic_auth_handler() -> &'static str {
    "Basic Auth Validated!"
}

/// A handler that is only accessible with a valid API key.
async fn api_key_handler() -> &'static str {
    "API Key Validated!"
}

/// A handler that is only accessible with a valid JWT token.
async fn jwt_handler() -> &'static str {
    "JWT Validated!"
}

#[derive(Debug, Deserialize)]
struct GenBasicAuthPayload {
    username: String,
    password: String,
}

async fn gen_basic_auth_handler(Json(payload): Json<GenBasicAuthPayload>) -> String {
    // This handler would generate a Basic Auth token.
    let token = utils::basic::create_basic_auth_token(&payload.username, &payload.password);
    tracing::info!("Generated Basic Auth Token: {}", token);
    token
}


#[derive(Debug, Deserialize)]
struct GenJwtPayload {
    username: String,
    #[serde(rename = "userType")]
    user_type: String,
}

async fn gen_jwt_handler(Json(payload): Json<GenJwtPayload>) -> String {
    // This handler would generate a JWT token.
    let token = utils::jwt::create_jwt_token(payload.username.clone(), payload.user_type.clone());
    tracing::info!("Generated JWT Token: {}", token);
    token
}


async fn user_login_handler(Form(payload): Form<GenBasicAuthPayload>) -> (StatusCode, String) {
    // This handler would process user login.
    tracing::info!("User login attempt: {:?}", payload);

    // Validate user and password
    if payload.username != config::BASIC_AUTH_USERNAME || payload.password != config::BASIC_AUTH_PASSWORD {
        tracing::warn!("Invalid login attempt for user: {}", payload.username);
        return (StatusCode::UNAUTHORIZED, "Invalid username or password".to_string());
    }

    // Generate JWT token for the user.
    let token = utils::jwt::create_jwt_token(payload.username.clone(), "user".to_string());
    tracing::info!("Generated JWT Token for user: {}", token);
    (StatusCode::OK, token)
}
