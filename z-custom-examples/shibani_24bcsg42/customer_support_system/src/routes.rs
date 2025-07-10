use axum::{
    Router,
    routing::{post, put, delete, get},
}
use crate::handlers::{
    customers,
    users,
    tickets,
    communications,
    knowledge_base,
    tags,
    analytics,
    audit_logs,
};
use crate::app_state::AppState;


pub fn routes() -> Router<AppState> {
    Router::new()
         // ---------- Users ----------
        .route("/users", post(users::create_user).get(users::get_users))
        .route("/users/:id", put(users::update_user).delete(users::delete_user))

        // ---------- Customers ----------
        .route("/customers", post(customers::create_customer).get(customers::get_customers))
        .route("/customers/:id", put(customers::update_customer).delete(customers::delete_customer))

        // ---------- Tickets ----------
        .route("/tickets", post(tickets::create_ticket).get(tickets::get_tickets))
        .route("/tickets/:id", put(tickets::update_ticket).delete(tickets::delete_ticket))

        // ---------- Communications ----------
        .route("/communications", post(communications::create_communication).get(communications::get_communications))
        .route("/communications/:id", delete(communications::delete_communication))

        // ---------- Knowledge Base ----------
        .route("/kb", post(knowledge_base::create_article).get(knowledge_base::get_articles))
        .route("/kb/:id", put(knowledge_base::update_article).delete(knowledge_base::delete_article))

        // ---------- Tags ----------
        .route("/tags", post(tags::create_tag).get(tags::get_tags))
        .route("/tags/:id", delete(tags::delete_tag))

        // ---------- Analytics ----------
        .route("/analytics", post(analytics::create_analytics).get(analytics::get_analytics))
        .route("/analytics/:id", put(analytics::update_analytics).delete(analytics::delete_analytics))

        // ---------- Audit Logs ----------
        .route("/audit-logs", post(audit_logs::create_log).get(audit_logs::get_logs))
        .route("/audit-logs/:id", delete(audit_logs::delete_log))

}