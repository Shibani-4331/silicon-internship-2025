use utoipa::OpenApi;
use crate::api;


#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::create_user,
        crate::api::login_user,
        crate::api::get_users,
        crate::api::create_customer,
        crate::api::get_customers,
        crate::api::create_ticket,
        crate::api::get_ticket_by_id,
        crate::api::get_all_tickets,
        crate::api::update_ticket_priority,
        crate::api::update_ticket_status,
        crate::api::delete_ticket_by_id,
        crate::api::assign_ticket,
        crate::api::get_filtered_tickets,
        crate::api::create_communication,
        crate::api::get_communications,
        crate::api::create_article,
        crate::api::update_article,
        crate::api::get_all_articles,
        crate::api::delete_article,
        crate::api::search_articles,
        crate::api::create_tag,
        crate::api::get_tags_by_id,
        crate::api::delete_tag,
        crate::api::create_analytics,
        crate::api::get_analytics,
        crate::api::get_analytics_by_id

    ),
    components(
        schemas  (
           api::CreateUserInput, 
           api::UserResponse, 
           api::LoginInput, 
           api::LoginResponse, 
           api::CreateCustomerInput, 
           api::CustomerResponse, 
           api::CreateTicketInput, 
           api::TicketResponse, 
           api::CreateCommunicationInput, 
           api::CommunicationResponse, 
           api::CreateArticleInput, 
           api::ArticleResponse, 
           api::SearchQuery,
           api::StatusInput, 
           api::PriorityInput, 
           api::AssignInput,
           api::TicketQuery,
           api::CreateTagInput,
           api::TagResponse,
           api::CreateAnalyticsInput,    
           api::AnalyticsResponse,
        )
    ),
    tags(
        (name = "User", description = "User endpoints"),
        (name = "Customer", description = "Customer endpoints"),
        (name = "Ticket", description = "Ticket endpoints"),
        (name = "Communication", description = "Communication endpoints"),
        (name = "Knowledge", description = "Knowledge base endpoints"),
        (name = "Tag", description = "Tag endpoints"),
        (name = "Analytics", description = "Analytics endpoints"),
    )
)]
pub struct ApiDoc;