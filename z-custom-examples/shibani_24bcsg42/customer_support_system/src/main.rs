use axum::{
    Router,
};
use std::env;
use sea_orm::{Database};
use dotenvy::dotenv;
use crate::app_state::AppState;
use std::sync::Arc;

mod routes; 
mod app_state;
mod entity;
mod API;
use entity::prelude::*;




#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = env::var("DB_url").expect("DATABASE_URL must be set");
    let db = Database::connect(&db_url)
        .await
        .expect("Failed to connect to database");


    let state = AppState {
        db: Arc::new(db)
    };

    let app = Router::new()
    .merge(routes::routes())
    .with_state(state);


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("server running on {} ", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
