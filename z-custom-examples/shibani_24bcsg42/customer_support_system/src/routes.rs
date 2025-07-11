use axum::{
    Router,
    routing::{post, put, delete, get}
};
use crate::API;
use crate::app_state::AppState;


pub fn routes() -> Router<AppState> {
    Router::new()
         // ---------- Users ----------
        .route("/users", post(create_user).get(get_users))
        .route("/users/:id", put(update_user).delete(delete_user))

        // ---------- Customers ----------
        .route("/customers", post(create_customer).get(get_customers))
        .route("/customers/:id", put(update_customer).delete(delete_customer))

        // ---------- Tickets ----------
        .route("/tickets", post(create_ticket).get(get_tickets))
        .route("/tickets/:id", put(update_ticket).delete(delete_ticket))

        // ---------- Communications ----------
        .route("/communications", post(create_communication).get(get_communications))
        .route("/communications/:id", delete(delete_communication))

        // ---------- Knowledge Base ----------
        .route("/kb", post(create_article).get(get_articles))
        .route("/kb/:id", put(update_article).delete(delete_article))

        // ---------- Tags ----------
        .route("/tags", post(create_tag).get(get_tags))
        .route("/tags/:id", delete(delete_tag))

        // ---------- Analytics ----------
        .route("/analytics", post(create_analytics).get(get_analytics))
        .route("/analytics/:id", put(update_analytics).delete(delete_analytics))

        // ---------- Audit Logs ----------
        .route("/audit-logs", post(create_log).get(get_logs))
        .route("/audit-logs/:id", delete(delete_log))

}