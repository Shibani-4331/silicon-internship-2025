use axum::{
    Router,
    routing::{post, put, delete, get, patch },
    extract::{State, Json},
};
use crate::api::{
    create_user, get_users, update_user, delete_user,
    create_customer, get_customers, update_customer, delete_customer,
    create_ticket, get_tickets, update_ticket, delete_ticket,
    create_communication, get_communications, delete_communication,
    create_article, get_articles, update_article, delete_article,
    create_tag, get_tags, delete_tag,
    create_analytics, get_analytics, update_analytics, delete_analytics,
    create_log, get_logs, delete_log,
    login_user,
    root_handler,
    assign_ticket
};
use crate::app_state::AppState;
use crate::auth::{AuthUser, require_role};



pub fn routes() -> Router<AppState> {
    Router::new()

        .route("/", get(root_handler))
        // ---------- Authentication ----------
        .route("/login", post(login_user))
        // ---------- Users ----------
        .route("/users", post(create_user).get(get_users))
        .route("/users/id", put(update_user).delete(delete_user))

        // ---------- Customers ----------
        .route("/customers", post(create_customer).get(get_customers))
        .route("/customers/id", put(update_customer).delete(delete_customer))

        // ---------- Tickets ----------
        .route("/tickets", post(create_ticket).get(get_tickets))
        .route("/tickets/id", put(update_ticket).delete(delete_ticket))
        .route("/tickets/id/assign", patch(assign_ticket))

        // ---------- Communications ----------
        .route("/communications", post(create_communication).get(get_communications))
        .route("/communications/id", delete(delete_communication))

        // ---------- Knowledge Base ----------
        .route("/kb", post(create_article).get(get_articles))
        .route("/kb/id", put(update_article).delete(delete_article))

        // ---------- Tags ----------
        .route("/tags", post(create_tag).get(get_tags))
        .route("/tags/id", delete(delete_tag))

        // ---------- Analytics ----------
        .route("/analytics", post(create_analytics).get(get_analytics))
        .route("/analytics/id", put(update_analytics).delete(delete_analytics))

        // ---------- Audit Logs ----------
        .route("/audit-logs", post(create_log).get(get_logs))
        .route("/audit-logs/id", delete(delete_log))

}