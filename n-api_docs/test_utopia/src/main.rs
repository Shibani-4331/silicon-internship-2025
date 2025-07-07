use std::net::{Ipv4Addr, SocketAddr};

use std::io::Error;
use tokio::net::TcpListener;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

mod docs;
mod todo;

use docs::ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/api/v1/todos", todo::router::router())
        .split_for_parts();

    let router = router
        .merge(
            SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", api.clone())
        );


    let address = SocketAddr::from((Ipv4Addr::UNSPECIFIED, 8080));
    let listener = TcpListener::bind(&address).await?;
    axum::serve(listener, router).await
}
