use axum::{body::Body, extract::Request, middleware::Next, response::{IntoResponse, Response}, Router};
use tower_http::trace::TraceLayer;
use tracing::{info, info_span};

// pub async fn log_request_raw(req: Request, next: Next) -> impl IntoResponse {
//     let method = req.method().to_string();
//     let path = req.uri().path().to_string();
//     let headers = req.headers().clone();

//     println!("Request: {} {}", method, path);
//     for (key, value) in headers.iter() {
//         println!("{}: {:?}", key, value);
//     }

//     // Call the next middleware or handler
//     next.run(req).await
// }


pub fn log_request_tracing(router: Router) -> Router {
    router.layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &Request<Body>| {
                info_span!(
                    "http_request",
                    method = %request.method(),
                    path = %request.uri().path(),
                    headers = ?request.headers(),
                )
            })
            .on_request(|_request: &Request<Body>, _span: &tracing::Span| {
                info!("Received request");
            })
            .on_response(|response: &axum::http::Response<Body>, _latency: std::time::Duration, _span: &tracing::Span| {
                info!("Response sent with status {}", response.status());
            })
    )
}