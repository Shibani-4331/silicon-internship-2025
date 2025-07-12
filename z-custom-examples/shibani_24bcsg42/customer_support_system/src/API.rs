use axum::{
    response::IntoResponse,
    extract::{State, Path},
    Router,
    http::StatusCode,
    Json,
};
use axum::debug_handler;
use chrono::{NaiveDate, Duration}; 
use sea_orm::{EntityTrait, Set, ActiveModelTrait, DatabaseConnection, QueryFilter, ColumnTrait, ModelTrait};
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::Utc;
use sea_orm::prelude::Uuid;
use crate::{app_state::AppState};
use crate::entity::prelude::*;
use crate::entity::users;
use crate::entity::customers;
use crate::entity::tickets;
use crate::entity::communications;
use crate::entity::knowledge_base;
use crate::entity::tags;
use crate::entity::analytics;   
use crate::entity::audit_logs;
use crate::entity::users::Entity as UserEntity;
use crate::entity::customers::Entity as CustomerEntity;
use crate::entity::tickets::Entity as TicketEntity;
use crate::entity::communications::Entity as CommunicationEntity;
use crate::entity::knowledge_base::Entity as KBEntity;
use crate::entity::tags::Entity as TagEntity;
use crate::entity::analytics::Entity as AnalyticsEntity;
use crate::entity::audit_logs::Entity as AuditLogEntity;
use crate::auth::{generate_jwt, AuthUser, require_role};


pub async fn root_handler() -> &'static str {
    "Welcome to the User API"
}

//-----------login--------------
#[derive(Deserialize)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn login_user(
    State(state): State<AppState>,
    Json(input): Json<LoginInput>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    use crate::entity::users;

    let user = users::Entity::find()
        .filter(users::Column::Email.eq(input.email.clone()))
        .one(state.db.as_ref())
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()))?
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid email".into()))?;


    if input.password != user.password_hash {
        return Err((StatusCode::UNAUTHORIZED, "Invalid password".into()));
    }

    let token = crate::auth::generate_jwt(&user.id.to_string(), &user.role);

    Ok(Json(LoginResponse { token }))
}

//----------user----------------
#[derive(Deserialize)]
pub struct CreateUserInput {
    email: String,
    name: String,
    password_hash: String,
    role: String,
}
use sea_orm::IntoActiveModel;


#[derive(Serialize)]
pub struct UserResponse {
    id: Uuid,
    email: String,
    name: String,
    role: String,
}

 pub async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<CreateUserInput>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let user = users::ActiveModel {
        id: Set(Uuid::new_v4()),
        email: Set(input.email),
        name: Set(input.name),
        password_hash: Set(input.password_hash.to_string()),
        role: Set(input.role.to_string()),
        created_at: Set(Utc::now().into())
    };

    let db = &state.db;
    let res = users::ActiveModel::insert(user, db.as_ref())
        .await
        .map_err(|e| {
            eprintln!("Failed to create user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user".into())
    })?;

    Ok(Json(UserResponse {
        id: res.id,
        email: res.email,
        name: res.name,
        role: res.role,
    }))
}
use crate::auth;
pub async fn get_users(
    State(state): State<AppState>,
    _auth_user: auth::AuthUser,
) -> Result<Json<Vec<UserResponse>>, (StatusCode, String)> {
    require_role(&_auth_user, "admin")?;

    let db = &state.db;
    let users = UserEntity::find()
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
            role: user.role,
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
    use axum::http::StatusCode;

let user = UserEntity::find_by_id(id)
    .one(db.as_ref())
    .await
    .map_err(|err| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Database error: {}", err),
        )
    })?;


    let mut active_user: users::ActiveModel = user.unwrap().into();
    active_user.email = Set(input.email);
    active_user.name = Set(input.name);
    active_user.created_at = Set(Utc::now()); 

    let res = active_user.update(db.as_ref()).await.map_err(|e| {
        eprintln!("Failed to update user: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Update failed".into())
    })?;

    Ok(Json(UserResponse {
        id: res.id,
        email: res.email,
        name: res.name,
        role: res.role,
    }))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let db = &state.db;
    UserEntity::delete_by_id(id)
        .exec(db.as_ref())
        .await
        .map_err(|e| {
            eprintln!("Failed to delete user: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Deletion failed".into())
        })?;
    Ok(StatusCode::NO_CONTENT)
}

//----------customer----------------
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
    let res = customer.insert(db.as_ref()).await.map_err(|e| {
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
    _auth: AuthUser, // Assuming AuthUser is a middleware for authentication
) -> Result<Json<Vec<CustomerResponse>>, (StatusCode, String)> {
    let db = &state.db;
    let list = CustomerEntity::find().all(db.as_ref()).await.map_err(|e| {
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
    let record = CustomerEntity::find_by_id(id).one(db.as_ref()).await.map_err(|e| {
        eprintln!("Find error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not find customer".into())
    })?;

    let mut model = record.ok_or((StatusCode::NOT_FOUND, "Customer not found".into()))?.into_active_model();

    model.name = Set(input.name);
    model.email = Set(input.email);
    model.phone = Set(input.phone);

    let updated = model.update(db.as_ref()).await.map_err(|e| {
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
    CustomerEntity::delete_by_id(id).exec(db.as_ref()).await.map_err(|e| {
        eprintln!("Deletion error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not delete customer".into())
    })?;

    Ok(StatusCode::NO_CONTENT)
}

//----------ticket----------------
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

    let saved = ticket.insert(db.as_ref()).await.map_err(|e| {
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


//ASSIGNED_AGENT_ID
#[derive(Deserialize)]
pub struct AssignInput {
    pub agent_id: String,
}

pub async fn assign_ticket(
    Path(ticket_id): Path<String>,
    State(state): State<AppState>,
    auth: AuthUser,
    Json(input): Json<AssignInput>,
) -> Result<Json<String>, (StatusCode, String)> {
    require_role(&auth, "admin")?;

    let uuid = uuid::Uuid::parse_str(&ticket_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID".to_string()))?;

    let db = &state.db;
    let mut ticket = tickets::Entity::find_by_id(uuid)
        .one(db.as_ref())
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()))?
        .ok_or((StatusCode::NOT_FOUND, "Ticket not found".into()))?;

    ticket.assigned_agent_id = Some(uuid::Uuid::parse_str(&input.agent_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid agent UUID".into()))?);

    let mut active: tickets::ActiveModel = ticket.clone().into();
    active.assigned_agent_id = Set(ticket.assigned_agent_id);

    
    active
        .update(db.as_ref())
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Update failed".into()))?;

    Ok(Json("Agent assigned successfully".into()))
}

// READ ALL

fn can_read_or_edit_ticket(
    auth: &AuthUser,
    ticket: &tickets::Model,
) -> Result<(), (StatusCode, String)> {
    if auth.role == "admin" {
        return Ok(());
    }

    if auth.role == "agent"
        && ticket.assigned_agent_id
            .as_ref()
            .map(|id| id.to_string()) == Some(auth.u_id.to_string())
    {
        return Ok(());
    }

   if auth.role == "customer"
    && Some(auth.u_id.to_string()) == Some(ticket.customer_id.to_string())
    {
         return Ok(());
    }


    Err((StatusCode::FORBIDDEN, "Access denied".into()))
}


pub async fn get_ticket_by_id(
    Path(ticket_id): Path<String>,
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let id = Uuid::parse_str(&ticket_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID".into()))?;

    let db = &state.db;
    let ticket = tickets::Entity::find_by_id(id)
        .one(db.as_ref())
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into()))?
        .ok_or((StatusCode::NOT_FOUND, "Ticket not found".into()))?;


    can_read_or_edit_ticket(&auth, &ticket)?;

    Ok(Json(ticket))
}
// GET /tickets — Admin only
#[debug_handler]
pub async fn get_all_tickets (
    State(state): State<AppState>,
    auth: AuthUser
) -> Result<impl IntoResponse, (StatusCode, String)> {
    require_role(&auth, "admin")?;

    let db = &state.db;
    let all_tickets = tickets::Entity::find()
        .all(db.as_ref())
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Database error".into()))?;

    Ok(Json(all_tickets))
}


// UPDATE
#[derive(Deserialize)]
pub struct StatusInput {
    pub status: String,
}

#[derive(Deserialize)]
pub struct PriorityInput {
    pub priority: String,
}


fn can_edit_ticket(auth: &AuthUser, ticket: &tickets::Model) -> Result<(), (StatusCode, String)> {
    if auth.role == "admin" {
        return Ok(());
    }
    if auth.role == "agent" && Some(auth.u_id.clone()) == ticket.assigned_agent_id.map(|id| id.to_string()) {
        return Ok(());
    }
    Err((StatusCode::FORBIDDEN, "Not allowed to modify this ticket".into()))
}

// UPDATE status
pub async fn update_ticket_status(
    Path(ticket_id): Path<String>,
    State(state): State<AppState>,
    auth: AuthUser,
    Json(input): Json<StatusInput>,
) -> Result<Json<String>, (StatusCode, String)> {
    let id = uuid::Uuid::parse_str(&ticket_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID".into()))?;

    let db = &state.db;
    let ticket = tickets::Entity::find_by_id(id)
        .one(db.as_ref())
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()))?
        .ok_or((StatusCode::NOT_FOUND, "Ticket not found".into()))?;

    can_edit_ticket(&auth, &ticket)?;

    let mut active = ticket.into_active_model();
    active.status = Set(input.status);

    active
        .update(db.as_ref())
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update status".into()))?;

    Ok(Json("Status updated successfully".into()))
}

// UPDATE priority
pub async fn update_ticket_priority(
    Path(ticket_id): Path<String>,
    State(state): State<AppState>,
    auth: AuthUser,
    Json(input): Json<PriorityInput>,
) -> Result<Json<String>, (StatusCode, String)> {
    let id = uuid::Uuid::parse_str(&ticket_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID".into()))?;

    let db = &state.db;
    let ticket = tickets::Entity::find_by_id(id)
        .one(db.as_ref())
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()))?
        .ok_or((StatusCode::NOT_FOUND, "Ticket not found".into()))?;

    can_edit_ticket(&auth, &ticket)?;

    let mut active = ticket.into_active_model();
    active.priority = Set(input.priority);

    active
        .update(db.as_ref())
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to update priority".into()))?;

    Ok(Json("Priority updated successfully".into()))
}


// DELETE
fn can_delete_ticket(
    auth: &AuthUser,
    ticket: &tickets::Model,
) -> Result<(), (StatusCode, String)> {
    if auth.role == "admin" {
        return Ok(());
    }

    if auth.role == "agent"
        && Some(auth.u_id.clone()) == ticket.assigned_agent_id.map(|id| id.to_string())
    {
        return Ok(());
    }

    if auth.role == "customer"
        && Some(auth.u_id.to_string()) == Some(ticket.customer_id.to_string())

    {
        return Ok(());
    }

    Err((StatusCode::FORBIDDEN, "Access denied".into()))
}
pub async fn delete_ticket_by_id(
    Path(ticket_id): Path<String>,
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<StatusCode, (StatusCode, String)> {
    let uuid = Uuid::parse_str(&ticket_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid UUID".to_string()))?;

    let db = &state.db;
    let ticket = tickets::Entity::find_by_id(uuid)
        .one(db.as_ref())
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "DB error".into()))?
        .ok_or((StatusCode::NOT_FOUND, "Ticket not found".into()))?;

    can_delete_ticket(&auth, &ticket)?; 

    ticket
        .delete(db.as_ref())
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete".into()))?;

    Ok(StatusCode::NO_CONTENT)
}


//----------communication----------------
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

    let saved = model.insert(db.as_ref()).await.map_err(|e| {
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
     Path(ticket_id): Path<String>,
    State(state): State<AppState>,
    auth: AuthUser,
) -> Result<Json<Vec<CommunicationResponse>>, (StatusCode, String)> {
    let uuid = Uuid::parse_str(&ticket_id)
        .map_err(|_| (StatusCode::BAD_REQUEST, "Invalid ticket ID".into()))?;


    let db = &state.db;
    let list = CommunicationEntity::find().all(db.as_ref()).await.map_err(|e| {
        eprintln!("Fetch error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not fetch communications".into())
    })?;

    let visible = if auth.role == "customer" {
        list.into_iter().filter(|c| !c.is_internal).collect()
    } else {
        list
    };

    let response = visible.into_iter().map(|c| CommunicationResponse {
        id: c.id,
        ticket_id: c.ticket_id,
        sender_type: c.sender_type,
        sender_id: c.sender_id,
        message: c.message,
        channel: c.channel,
        is_internal: c.is_internal,
    }).collect();   
    Ok(Json(response))
}


//----------knowledge_base----------------
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

    let saved = article.insert(db.as_ref()).await.map_err(|e| {
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

    let list = KBEntity::find().all(db.as_ref()).await.map_err(|e| {
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

    let record = KBEntity::find_by_id(id).one(db.as_ref()).await.map_err(|e| {
        eprintln!("Find error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not find article".into())
    })?;

    let mut model = record.ok_or((StatusCode::NOT_FOUND, "Article not found".into()))?.into_active_model();

    model.title = Set(input.title);
    model.content = Set(input.content);
    model.category = Set(input.category);
    model.created_by = Set(input.created_by);

    let updated = model.update(db.as_ref()).await.map_err(|e| {
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

    KBEntity::delete_by_id(id).exec(db.as_ref()).await.map_err(|e| {
        eprintln!("Deletion error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not delete article".into())
    })?;

    Ok(StatusCode::NO_CONTENT)
}


//----------tag----------------
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

    let saved = tag.insert(db.as_ref()).await.map_err(|e| {
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

    let list = TagEntity::find().all(db.as_ref()).await.map_err(|e| {
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

    TagEntity::delete_by_id(id).exec(db.as_ref()).await.map_err(|e| {
        eprintln!("Delete error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not delete tag".into())
    })?;

    Ok(StatusCode::NO_CONTENT)
}


//----------analytics----------------
#[derive(Deserialize)]
pub struct CreateAnalyticsInput {
    pub date: NaiveDate,
    pub total_tickets: i32,
    pub resolved_tickets: i32,
    pub avg_response_time: i64,
    pub agent_id: Uuid,
}

#[derive(Serialize)]
pub struct AnalyticsResponse {
    pub id: Uuid,
    pub date: NaiveDate,
    pub total_tickets: i32,
    pub resolved_tickets: i32,
    pub avg_response_time: i64,
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

    let saved = analytics.insert(db.as_ref()).await.map_err(|e| {
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

    let list = AnalyticsEntity::find().all(db.as_ref()).await.map_err(|e| {
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

    let record = AnalyticsEntity::find_by_id(id).one(db.as_ref()).await.map_err(|e| {
        eprintln!("Find error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not find analytics".into())
    })?;

    let mut model = record.ok_or((StatusCode::NOT_FOUND, "Analytics not found".into()))?.into_active_model();

    model.date = Set(input.date);
    model.total_tickets = Set(input.total_tickets);
    model.resolved_tickets = Set(input.resolved_tickets);
    model.avg_response_time = Set(input.avg_response_time);
    model.agent_id = Set(input.agent_id);

    let updated = model.update(db.as_ref()).await.map_err(|e| {
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

    AnalyticsEntity::delete_by_id(id).exec(db.as_ref()).await.map_err(|e| {
        eprintln!("Deletion error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not delete analytics".into())
    })?;

    Ok(StatusCode::NO_CONTENT)
}


//----------audit_logs----------------
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
        ip_address: Set(input.ip_address.to_string()),
    };

    let saved = log.insert(db.as_ref()).await.map_err(|e| {
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

    let list = AuditLogEntity::find().all(db.as_ref()).await.map_err(|e| {
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

    AuditLogEntity::delete_by_id(id).exec(db.as_ref()).await.map_err(|e| {
        eprintln!("Deletion error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, "Could not delete log".into())
    })?;

    Ok(StatusCode::NO_CONTENT)
}