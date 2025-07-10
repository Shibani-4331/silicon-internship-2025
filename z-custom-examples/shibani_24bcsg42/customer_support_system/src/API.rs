use axum::{
    routing::{post, get},
    extract::{State},
    Router,
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, Set, ActiveModelTrait, DatabaseConnection};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use sqlx::types::chrono::Utc;
use sea_orm::prelude::Uuid;
use crate::entity::users::ActiveModel;
use crate::{app_state::AppState, entity::users};

pub use users::Entity as UserEntity;

#[derive(Deserialize)]
struct CreateUserInput {
    email: String,
    name: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: Uuid,
    email: String,
    name: String,
}



pub async fn root_handler() -> &'static str {
    "Welcome to the User API"
}

 pub async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<CreateUserInput>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = users::ActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set(input.email),
        name: Set(input.name),
        created_at:Set(Utc::now().into())
    };

    let db = &state.db;
    let res = ActiveModel::insert(user, db.as_ref())
        .await
        .map_err(|e| {
            eprintln!("Failed to create user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user".into())
    })?;

    Ok(Json(UserResponse {
        id: res.id,
        email: res.email,
        name: res.name,
    }))
}

pub async fn get_users(
    State(state): State<AppState>,
) -> Result<Json<Vec<UserResponse>>, (StatusCode, String)> {
    let db = &state.db;
    let users = users::Entity::find()
        .all(db.as_ref())
        .await
        .map_err(|e| {
            eprintln!("Failed to retrieve users: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to retrieve users".into())
        })?;

     let response = users
        .into_iter()
        .map(|user| UserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
        })
        .collect();

    Ok(Json(response))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<CreateUserInput>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let db = &state.db;
    let mut user = UserEntity::find_by_id(id)
        .await?
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?
        .into_active_model();

    user.email = Set(input.email);
    user.name = Set(input.name);
    user.updated_at = Set(Utc::now().into());

    let res = user.update(db).await.map_err(|e| {
        eprintln!("Failed to update user: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Update failed".into())
    })?;

    Ok(Json(UserResponse {
        id: res.id,
        email: res.email,
        name: res.name,
    }))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = &state.db;
    UserEntity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(|e| {
            eprintln!("Failed to delete user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Deletion failed".into())
        })?;
    Ok(StatusCode::NO_CONTENT)
}


#[derive(Deserialize)]
pub struct CreateCustomerInput {
    pub name: String,
    pub email: String,
    pub phone: String,
}

#[derive(Serialize)]
pub struct CustomerResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: String,
}

// CREATE
pub async fn create_customer(
    State(state): State<AppState>,
    Json(input): Json<CreateCustomerInput>,
) -> Result<Json<CustomerResponse>, (StatusCode, String)> {
    let customer = customers::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(input.name),
        email: Set(input.email),
        phone: Set(input.phone),
        created_at: Set(Utc::now().into()),
    };

    let db = &state.db;
    let res = customer.insert(db).await.map_err(|e| {
        eprintln!("Error creating customer: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not create customer".into())
    })?;

    Ok(Json(CustomerResponse {
        id: res.id,
        name: res.name,
        email: res.email,
        phone: res.phone,
    }))
}

// READ ALL
pub async fn get_customers(
    State(state): State<AppState>,
) -> Result<Json<Vec<CustomerResponse>>, (StatusCode, String)> {
    let db = &state.db;
    let list = CustomerEntity::find().all(db).await.map_err(|e| {
        eprintln!("Error fetching customers: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not fetch customers".into())
    })?;

    let response = list.into_iter().map(|c| CustomerResponse {
        id: c.id,
        name: c.name,
        email: c.email,
        phone: c.phone,
    }).collect();

    Ok(Json(response))
}

// UPDATE
pub async fn update_customer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<CreateCustomerInput>,
) -> Result<Json<CustomerResponse>, (StatusCode, String)> {
    let db = &state.db;
    let record = CustomerEntity::find_by_id(id).one(db).await.map_err(|e| {
        eprintln!("Find error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not find customer".into())
    })?;

    let mut model = record.ok_or((StatusCode::NOT_FOUND, "Customer not found".into()))?.into_active_model();

    model.name = Set(input.name);
    model.email = Set(input.email);
    model.phone = Set(input.phone);

    let updated = model.update(db).await.map_err(|e| {
        eprintln!("Update error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not update customer".into())
    })?;

    Ok(Json(CustomerResponse {
        id: updated.id,
        name: updated.name,
        email: updated.email,
        phone: updated.phone,
    }))
}

// DELETE
pub async fn delete_customer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = &state.db;
    CustomerEntity::delete_by_id(id).exec(db).await.map_err(|e| {
        eprintln!("Deletion error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not delete customer".into())
    })?;

    Ok(StatusCode::NO_CONTENT)
}

use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, ActiveModelTrait, Set};
use sea_orm::prelude::Uuid;
use sqlx::types::chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::{AppState, entity::tickets};
pub use tickets::Entity as TicketEntity;

#[derive(Deserialize)]
pub struct CreateTicketInput {
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub channel: String,
    pub customer_id: Uuid,
    pub assigned_agent_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct TicketResponse {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub channel: String,
    pub customer_id: Uuid,
    pub assigned_agent_id: Option<Uuid>,
}

// CREATE
pub async fn create_ticket(
    State(state): State<AppState>,
    Json(input): Json<CreateTicketInput>,
) -> Result<Json<TicketResponse>, (StatusCode, String)> {
    let db = &state.db;
    let ticket = tickets::ActiveModel {
        id: Set(Uuid::new_v4()),
        title: Set(input.title),
        description: Set(input.description),
        status: Set(input.status),
        priority: Set(input.priority),
        channel: Set(input.channel),
        customer_id: Set(input.customer_id),
        assigned_agent_id: Set(input.assigned_agent_id),
        created_at: Set(Utc::now().into()),
        updated_at: Set(Utc::now().into()),
    };

    let saved = ticket.insert(db).await.map_err(|e| {
        eprintln!("Failed to insert ticket: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Ticket creation failed".into())
    })?;

    Ok(Json(TicketResponse {
        id: saved.id,
        title: saved.title,
        description: saved.description,
        status: saved.status,
        priority: saved.priority,
        channel: saved.channel,
        customer_id: saved.customer_id,
        assigned_agent_id: saved.assigned_agent_id,
    }))
}

// READ ALL
pub async fn get_tickets(
    State(state): State<AppState>,
) -> Result<Json<Vec<TicketResponse>>, (StatusCode, String)> {
    let db = &state.db;
    let tickets = TicketEntity::find().all(db).await.map_err(|e| {
        eprintln!("Failed to fetch tickets: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Fetch error".into())
    })?;

    let response = tickets.into_iter().map(|t| TicketResponse {
        id: t.id,
        title: t.title,
        description: t.description,
        status: t.status,
        priority: t.priority,
        channel: t.channel,
        customer_id: t.customer_id,
        assigned_agent_id: t.assigned_agent_id,
    }).collect();

    Ok(Json(response))
}

// UPDATE
pub async fn update_ticket(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<CreateTicketInput>,
) -> Result<Json<TicketResponse>, (StatusCode, String)> {
    let db = &state.db;
    let record = TicketEntity::find_by_id(id).one(db).await.map_err(|e| {
        eprintln!("Fetch failed: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Ticket not found".into())
    })?;

    let mut model = record.ok_or((StatusCode::NOT_FOUND, "No ticket with that ID".into()))?.into_active_model();

    model.title = Set(input.title);
    model.description = Set(input.description);
    model.status = Set(input.status);
    model.priority = Set(input.priority);
    model.channel = Set(input.channel);
    model.customer_id = Set(input.customer_id);
    model.assigned_agent_id = Set(input.assigned_agent_id);
    model.updated_at = Set(Utc::now().into());

    let updated = model.update(db).await.map_err(|e| {
        eprintln!("Update failed: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not update".into())
    })?;

    Ok(Json(TicketResponse {
        id: updated.id,
        title: updated.title,
        description: updated.description,
        status: updated.status,
        priority: updated.priority,
        channel: updated.channel,
        customer_id: updated.customer_id,
        assigned_agent_id: updated.assigned_agent_id,
    }))
}

// DELETE
pub async fn delete_ticket(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = &state.db;
    TicketEntity::delete_by_id(id).exec(db).await.map_err(|e| {
        eprintln!("Delete failed: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not delete ticket".into())
    })?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct CreateCommunicationInput {
    pub ticket_id: Uuid,
    pub sender_type: String, // "agent" or "customer"
    pub sender_id: Uuid,
    pub message: String,
    pub channel: String,     // "Email", "Chat", "Social"
    pub is_internal: bool,
}

#[derive(Serialize)]
pub struct CommunicationResponse {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub sender_type: String,
    pub sender_id: Uuid,
    pub message: String,
    pub channel: String,
    pub is_internal: bool,
}

// CREATE
pub async fn create_communication(
    State(state): State<AppState>,
    Json(input): Json<CreateCommunicationInput>,
) -> Result<Json<CommunicationResponse>, (StatusCode, String)> {
    let db = &state.db;

    if !["agent", "customer"].contains(&input.sender_type.as_str()) {
        return Err((StatusCode::BAD_REQUEST, "Invalid sender_type".into()));
    }
    if !["Email", "Chat", "Social"].contains(&input.channel.as_str()) {
        return Err((StatusCode::BAD_REQUEST, "Invalid channel".into()));
    }

    let model = communications::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(input.ticket_id),
        sender_type: Set(input.sender_type.clone()),
        sender_id: Set(input.sender_id),
        message: Set(input.message.clone()),
        channel: Set(input.channel.clone()),
        is_internal: Set(input.is_internal),
        timestamp: Set(Utc::now().into()),
    };

    let saved = model.insert(db).await.map_err(|e| {
        eprintln!("Create error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Insert failed".into())
    })?;

    Ok(Json(CommunicationResponse {
        id: saved.id,
        ticket_id: saved.ticket_id,
        sender_type: saved.sender_type,
        sender_id: saved.sender_id,
        message: saved.message,
        channel: saved.channel,
        is_internal: saved.is_internal,
    }))
}

// READ ALL
pub async fn get_communications(
    State(state): State<AppState>,
) -> Result<Json<Vec<CommunicationResponse>>, (StatusCode, String)> {
    let db = &state.db;
    let list = CommunicationEntity::find().all(db).await.map_err(|e| {
        eprintln!("Fetch error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not fetch communications".into())
    })?;

    let response = list.into_iter().map(|m| CommunicationResponse {
        id: m.id,
        ticket_id: m.ticket_id,
        sender_type: m.sender_type,
        sender_id: m.sender_id,
        message: m.message,
        channel: m.channel,
        is_internal: m.is_internal,
    }).collect();

    Ok(Json(response))
}

// DELETE
pub async fn delete_communication(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = &state.db;
    CommunicationEntity::delete_by_id(id).exec(db).await.map_err(|e| {
        eprintln!("Delete error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not delete communication".into())
    })?;
    Ok(StatusCode::NO_CONTENT)
}


#[derive(Deserialize)]
pub struct CreateArticleInput {
    pub title: String,
    pub content: String,
    pub category: String,
    pub created_by: Uuid,
}

#[derive(Serialize)]
pub struct ArticleResponse {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub category: String,
    pub created_by: Uuid,
}

// CREATE
pub async fn create_article(
    State(state): State<AppState>,
    Json(input): Json<CreateArticleInput>,
) -> Result<Json<ArticleResponse>, (StatusCode, String)> {
    let db = &state.db;

    let article = knowledge_base::ActiveModel {
        id: Set(Uuid::new_v4()),
        title: Set(input.title),
        content: Set(input.content),
        category: Set(input.category),
        created_by: Set(input.created_by),
        created_at: Set(Utc::now().into()),
    };

    let saved = article.insert(db).await.map_err(|e| {
        eprintln!("Insert error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not create article".into())
    })?;

    Ok(Json(ArticleResponse {
        id: saved.id,
        title: saved.title,
        content: saved.content,
        category: saved.category,
        created_by: saved.created_by,
    }))
}

// GET ALL
pub async fn get_articles(
    State(state): State<AppState>,
) -> Result<Json<Vec<ArticleResponse>>, (StatusCode, String)> {
    let db = &state.db;

    let list = KBEntity::find().all(db).await.map_err(|e| {
        eprintln!("Fetch error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not fetch articles".into())
    })?;

    let response = list.into_iter().map(|article| ArticleResponse {
        id: article.id,
        title: article.title,
        content: article.content,
        category: article.category,
        created_by: article.created_by,
    }).collect();

    Ok(Json(response))
}

// UPDATE
pub async fn update_article(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<CreateArticleInput>,
) -> Result<Json<ArticleResponse>, (StatusCode, String)> {
    let db = &state.db;

    let record = KBEntity::find_by_id(id).one(db).await.map_err(|e| {
        eprintln!("Find error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not find article".into())
    })?;

    let mut model = record.ok_or((StatusCode::NOT_FOUND, "Article not found".into()))?.into_active_model();

    model.title = Set(input.title);
    model.content = Set(input.content);
    model.category = Set(input.category);
    model.created_by = Set(input.created_by);

    let updated = model.update(db).await.map_err(|e| {
        eprintln!("Update error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not update article".into())
    })?;

    Ok(Json(ArticleResponse {
        id: updated.id,
        title: updated.title,
        content: updated.content,
        category: updated.category,
        created_by: updated.created_by,
    }))
}

// DELETE
pub async fn delete_article(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = &state.db;

    KBEntity::delete_by_id(id).exec(db).await.map_err(|e| {
        eprintln!("Deletion error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not delete article".into())
    })?;

    Ok(StatusCode::NO_CONTENT)
}

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::{EntityTrait, ActiveModelTrait, Set};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};
use crate::{AppState, entity::tags};
pub use tags::Entity as TagEntity;

#[derive(Deserialize)]
pub struct CreateTagInput {
    pub ticket_id: Uuid,
    pub tag_name: String,
}

#[derive(Serialize)]
pub struct TagResponse {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub tag_name: String,
}

// CREATE
pub async fn create_tag(
    State(state): State<AppState>,
    Json(input): Json<CreateTagInput>,
) -> Result<Json<TagResponse>, (StatusCode, String)> {
    let db = &state.db;

    let tag = tags::ActiveModel {
        id: Set(Uuid::new_v4()),
        ticket_id: Set(input.ticket_id),
        tag_name: Set(input.tag_name.clone()),
    };

    let saved = tag.insert(db).await.map_err(|e| {
        eprintln!("Insert error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not create tag".into())
    })?;

    Ok(Json(TagResponse {
        id: saved.id,
        ticket_id: saved.ticket_id,
        tag_name: saved.tag_name,
    }))
}

// READ ALL
pub async fn get_tags(
    State(state): State<AppState>,
) -> Result<Json<Vec<TagResponse>>, (StatusCode, String)> {
    let db = &state.db;

    let list = TagEntity::find().all(db).await.map_err(|e| {
        eprintln!("Fetch error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not fetch tags".into())
    })?;

    let response = list.into_iter().map(|t| TagResponse {
        id: t.id,
        ticket_id: t.ticket_id,
        tag_name: t.tag_name,
    }).collect();

    Ok(Json(response))
}

// DELETE
pub async fn delete_tag(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = &state.db;

    TagEntity::delete_by_id(id).exec(db).await.map_err(|e| {
        eprintln!("Delete error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not delete tag".into())
    })?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct CreateAnalyticsInput {
    pub date: NaiveDate,
    pub total_tickets: i32,
    pub resolved_tickets: i32,
    pub avg_response_time: Duration,
    pub agent_id: Uuid,
}

#[derive(Serialize)]
pub struct AnalyticsResponse {
    pub id: Uuid,
    pub date: NaiveDate,
    pub total_tickets: i32,
    pub resolved_tickets: i32,
    pub avg_response_time: Duration,
    pub agent_id: Uuid,
}

// CREATE
pub async fn create_analytics(
    State(state): State<AppState>,
    Json(input): Json<CreateAnalyticsInput>,
) -> Result<Json<AnalyticsResponse>, (StatusCode, String)> {
    let db = &state.db;

    let analytics = analytics::ActiveModel {
        id: Set(Uuid::new_v4()),
        date: Set(input.date),
        total_tickets: Set(input.total_tickets),
        resolved_tickets: Set(input.resolved_tickets),
        avg_response_time: Set(input.avg_response_time),
        agent_id: Set(input.agent_id),
    };

    let saved = analytics.insert(db).await.map_err(|e| {
        eprintln!("Error creating analytics: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Insert failed".into())
    })?;

    Ok(Json(AnalyticsResponse {
        id: saved.id,
        date: saved.date,
        total_tickets: saved.total_tickets,
        resolved_tickets: saved.resolved_tickets,
        avg_response_time: saved.avg_response_time,
        agent_id: saved.agent_id,
    }))
}

// READ ALL
pub async fn get_analytics(
    State(state): State<AppState>,
) -> Result<Json<Vec<AnalyticsResponse>>, (StatusCode, String)> {
    let db = &state.db;

    let list = AnalyticsEntity::find().all(db).await.map_err(|e| {
        eprintln!("Fetch error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not fetch analytics".into())
    })?;

    let response = list.into_iter().map(|a| AnalyticsResponse {
        id: a.id,
        date: a.date,
        total_tickets: a.total_tickets,
        resolved_tickets: a.resolved_tickets,
        avg_response_time: a.avg_response_time,
        agent_id: a.agent_id,
    }).collect();

    Ok(Json(response))
}

// UPDATE
pub async fn update_analytics(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(input): Json<CreateAnalyticsInput>,
) -> Result<Json<AnalyticsResponse>, (StatusCode, String)> {
    let db = &state.db;

    let record = AnalyticsEntity::find_by_id(id).one(db).await.map_err(|e| {
        eprintln!("Find error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not find analytics".into())
    })?;

    let mut model = record.ok_or((StatusCode::NOT_FOUND, "Analytics not found".into()))?.into_active_model();

    model.date = Set(input.date);
    model.total_tickets = Set(input.total_tickets);
    model.resolved_tickets = Set(input.resolved_tickets);
    model.avg_response_time = Set(input.avg_response_time);
    model.agent_id = Set(input.agent_id);

    let updated = model.update(db).await.map_err(|e| {
        eprintln!("Update error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not update analytics".into())
    })?;

    Ok(Json(AnalyticsResponse {
        id: updated.id,
        date: updated.date,
        total_tickets: updated.total_tickets,
        resolved_tickets: updated.resolved_tickets,
        avg_response_time: updated.avg_response_time,
        agent_id: updated.agent_id,
    }))
}

// DELETE
pub async fn delete_analytics(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = &state.db;

    AnalyticsEntity::delete_by_id(id).exec(db).await.map_err(|e| {
        eprintln!("Deletion error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not delete analytics".into())
    })?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
pub struct CreateAuditLogInput {
    pub user_id: Uuid,
    pub action: String,
    pub entity: String,
    pub entity_id: Uuid,
    pub ip_address: String,
}

#[derive(Serialize)]
pub struct AuditLogResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub entity: String,
    pub entity_id: Uuid,
    pub timestamp: chrono::DateTime<Utc>,
    pub ip_address: String,
}

// CREATE
pub async fn create_log(
    State(state): State<AppState>,
    Json(input): Json<CreateAuditLogInput>,
) -> Result<Json<AuditLogResponse>, (StatusCode, String)> {
    let db = &state.db;

    let log = audit_logs::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(input.user_id),
        action: Set(input.action.clone()),
        entity: Set(input.entity.clone()),
        entity_id: Set(input.entity_id),
        timestamp: Set(Utc::now().into()),
        ip_address: Set(input.ip_address.clone().parse().unwrap_or(Ipv4Addr::LOCALHOST)),
    };

    let saved = log.insert(db).await.map_err(|e| {
        eprintln!("Insert error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not create log".into())
    })?;

    Ok(Json(AuditLogResponse {
        id: saved.id,
        user_id: saved.user_id,
        action: saved.action,
        entity: saved.entity,
        entity_id: saved.entity_id,
        timestamp: saved.timestamp,
        ip_address: saved.ip_address.to_string(),
    }))
}

// READ ALL
pub async fn get_logs(
    State(state): State<AppState>,
) -> Result<Json<Vec<AuditLogResponse>>, (StatusCode, String)> {
    let db = &state.db;

    let list = AuditLogEntity::find().all(db).await.map_err(|e| {
        eprintln!("Fetch error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not fetch logs".into())
    })?;

    let response = list.into_iter().map(|log| AuditLogResponse {
        id: log.id,
        user_id: log.user_id,
        action: log.action,
        entity: log.entity,
        entity_id: log.entity_id,
        timestamp: log.timestamp,
        ip_address: log.ip_address.to_string(),
    }).collect();

    Ok(Json(response))
}

// DELETE
pub async fn delete_log(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = &state.db;

    AuditLogEntity::delete_by_id(id).exec(db).await.map_err(|e| {
        eprintln!("Deletion error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not delete log".into())
    })?;

    Ok(StatusCode::NO_CONTENT)
}