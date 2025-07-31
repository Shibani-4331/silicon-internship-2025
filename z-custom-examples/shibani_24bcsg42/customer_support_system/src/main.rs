use axum::{
    Router,
};
use std::env;
use sea_orm::{Database};
use dotenvy::dotenv;
use crate::app_state::AppState;
use std::sync::Arc;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
pub use crate::doc::ApiDoc;
// use error_handle::AppError;

mod routes; 
mod app_state;
mod entity;
mod api;
mod auth;
mod doc;
mod error_handle;




#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DB_url").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url)
        .await
        .expect("Failed to connect to the database");


    let state = AppState {
        db: Arc::new(db)
    };

    let app = Router::new()
    .merge(SwaggerUi::new("/").url("/api-doc/openapi.json", ApiDoc::openapi()))
    .merge(routes::routes())
    .with_state(state);


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("server running on {} ", listener.local_addr().unwrap());
    println!("Swagger UI available at http://{}/", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
