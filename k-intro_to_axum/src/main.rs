use std::collections::HashMap;

use axum::{body::Body, extract::{Path, Query, Request}, response::{IntoResponse, Response}, routing::{delete, get, options, patch, post, put}, Json, Router};

// Dynamic Path: http://localhost:3000/silicon/student/<sic>

// QUery Parameters: http://localhost:3000/silicon/student?sic=123&branch=cs

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(hello))
        .route("/silicon", get(silicon))
        .route("/silicon/student", get(silicon_student_query_handler)) // Student List
        .route("/submit", post(submit_form_handler))
        .route("/silicon/student/{sic}/{branch}", get(silicon_student_details_handler))
        .fallback(handle_not_found);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_not_found() -> &'static str {
    "Not Found"
}

async fn handler() -> &'static str {
    "Hello, World!"
}


// async fn silicon() -> &'static str {
//     "Hello, Silicon!"
// }

async fn silicon() -> Json<serde_json::Value> {
    let response = serde_json::json!({
        "message": 1,
        "status": "success"
    });
    Json(response)
}

async fn silicon_student() -> &'static str {
    "Hello, Silicon Student!"
}


async fn submission_handler() -> &'static str {
    "Submission received!"
}

// Axum Extractor: Path - 1 Path parameter
// async fn silicon_student_details_handler(Path(sic): Path<String>) -> String {
//     format!("Hello, Silicon Student with SIC: {}", sic)
// }

// Path Parameters
async fn silicon_student_details_handler(Path((sic, branch)): Path<(String, String)>) -> String {
    format!("Hello, Silicon Student with SIC: {}, Branch: {}", sic, branch)
}


// Query Parameters
async fn silicon_student_query_handler(Query(params): Query<HashMap<String, String>>) -> String {
    // let student_sic: &str = params.get("sic").unwrap_or("NA");
    let student_sic = params.get("sic").unwrap();
    let student_branch = params.get("branch").unwrap();

    format!("Hello, Silicon Student with SIC: {}, Branch: {}", student_sic, student_branch)
}


// async fn submit_form(Json(payload): Json<User>) -> String {
//     // Process the payload
//     format!("Form submitted with data: {:?}", payload)

// }

#[derive(serde::Deserialize, serde::Serialize, Debug)]
struct User {
    name: String,
    age: u32,
}

async fn submit_form_handler(req: Request) -> &'static str {
    println!("Received request: {:?}", req);

    let body = axum::body::to_bytes(req.into_body(), 1024).await.unwrap();
    println!("Request body: {:?}", body);
    "Form submitted successfully!"
}

// async fn hello() -> Response {
//     let user = User {
//         name: "John Doe".to_string(),
//         age: 30,
//     };
//     let json_data = serde_json::to_string(&user).unwrap();
//     Response::new(Body::new(json_data))
// }

async fn hello() -> impl IntoResponse {
    let user = User {
        name: "John Doe".to_string(),
        age: 30,
    };
    let json_data = serde_json::to_string(&user).unwrap();
    (axum::http::StatusCode::CREATED, json_data)
}