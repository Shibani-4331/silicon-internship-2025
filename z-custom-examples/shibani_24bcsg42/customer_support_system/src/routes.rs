use axum::{
    Router,
    routing::{post, put, delete, get, patch },
    extract::{State, Json},
};
use crate::api::{
    //get_my_tickets, get_ticket_details, customer_reply_ticket,
    create_user, get_users, update_user, delete_user,
    create_customer, get_customers, update_customer, delete_customer,
    create_ticket, delete_ticket_by_id, update_ticket_priority, update_ticket_status, assign_ticket, get_ticket_by_id, get_all_tickets, get_filtered_tickets,
    create_communication, get_communications,
    create_article, get_all_articles, update_article, delete_article, search_articles,
    create_tag, get_tags_by_id, delete_tag,
    create_analytics, get_analytics, get_analytics_by_id,
    // create_log, get_logs, delete_log,
    login_user,
    // root_handler
};
use crate::app_state::AppState;
use crate::auth::{AuthUser, require_role};



pub fn routes() -> Router<AppState> {
    Router::new()

        
        // ---------- Authentication ----------
        .route("/login", post(login_user))
        // ---------- Users ----------
        .route("/users", post(create_user).get(get_users))
        .route("/users/id", put(update_user).delete(delete_user))

        // ---------- Customers ----------
        .route("/customers", post(create_customer).get(get_customers))
        .route("/customers/id", put(update_customer).delete(delete_customer))

        // ---------- Tickets ----------
        .route("/tickets", post(create_ticket))
        .route("/tickets", get(get_all_tickets))
        .route("/tickets/id", get(get_ticket_by_id))
        .route("/tickets/id", delete(delete_ticket_by_id))
        .route("/tickets/id/status", patch(update_ticket_status))
        .route("/tickets/id/priority", patch(update_ticket_priority))
        .route("/tickets/id/assign", patch(assign_ticket))
        .route("/tickets/search", get(get_filtered_tickets))

        // ---------- Communications ----------
        .route("/communications", post(create_communication))
        .route("/communications/id", get(get_communications))

        // ---------- Knowledge Base ----------
        .route("/kb", post(create_article))
        .route("/kb/id", put(update_article).delete(delete_article).get(get_all_articles))
        .route("/kb/search", get(search_articles))

        // ---------- Tags ----------
        .route("/tags", post(create_tag))
        .route("/tags/id", delete(delete_tag).get(get_tags_by_id))

        // ---------- Analytics ----------
        .route("/analytics", post(create_analytics).get(get_analytics))
        .route("/analytics/id", get(get_analytics_by_id))

        // // ---------- Audit Logs ----------
        // .route("/audit-logs", post(create_log).get(get_logs))
        // .route("/audit-logs/id", delete(delete_log))

        // // ---------- Customer Support ----------
        // .route("/api/customer/tickets", get(get_my_tickets))
        // .route("/api/customer/tickets/:ticket_id", get(get_ticket_details))
        // .route("/api/customer/tickets/:ticket_id/reply", post(customer_reply_ticket))

}