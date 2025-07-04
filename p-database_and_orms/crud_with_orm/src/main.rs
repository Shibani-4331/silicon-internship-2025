use std::net::{Ipv4Addr, SocketAddr};

use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;
use sea_orm::{sqlx::{self, postgres::PgPoolOptions}, Database};
use std::sync::Arc;
use dotenvy::dotenv;
use std::env;

mod docs;
mod todo;
mod entity;

use docs::ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Arc::new(Database::connect(&db_url).await?);

    let pool = PgPoolOptions::new()
        .connect(&db_url)
        .await?;

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS todos (
            id SERIAL PRIMARY KEY,
            title VARCHAR(255) NOT NULL,
            description TEXT,
            completed BOOLEAN DEFAULT FALSE,
            created_at TIMESTAMPTZ DEFAULT NOW()
        );
        "#
    )
    .execute(&pool)
    .await?;

    let _ = pool.close().await;

    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1/todos", todo::router::router(db.clone()))
        .split_for_parts();

    let router = router
        .merge(
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone())
        );


    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
