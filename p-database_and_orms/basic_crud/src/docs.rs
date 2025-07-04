use crate::todo::{self, handler};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, SecurityScheme},
    Modify, OpenApi,
};

#[derive(OpenApi)]
#[openapi(
    paths(
        handler::list_todos,
        handler::search_todos,
        handler::create_todo,
        handler::mark_done,
        handler::delete_todo,
    ),
    components(
        schemas(todo::model::Todo, todo::model::TodoError)
    ),
    modifiers(&SecurityAddon),
    security(
        ("api_key" = [])
    ),
    tags(
        (name = "todo", description = "Todo items management API")
    ),
    servers(
        (url = "/api/v1/todos", description = "Local Development Server")
    )
)]
pub struct ApiDoc;

pub struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("X-Api-Key"))),
            )
        }
    }
}

