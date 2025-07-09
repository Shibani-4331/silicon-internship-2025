use axum::{
    Router,
    routing::get,
};
use std::env;
use sea_orm::{Database};
use dotenvy::dotenv;


#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DB_url").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url)
        .await
        .expect("Failed to connect to database");


    let app = Router::new()
    .route("/", get(|| async { "server is running" }));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
