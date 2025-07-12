use sea_orm::entity::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "communications")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub sender_type: String,      // "agent" or "customer"
    pub sender_id: Uuid,
    pub message: String,
    pub channel: String,          // "Email", "Chat", "Social"
    pub is_internal: bool,
    pub timestamp: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Ticket,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Ticket => Entity::belongs_to(super::tickets::Entity)
                .from(Column::TicketId)
                .to(super::tickets::Column::Id)
                .into(),
        }
    }
}

impl Related<super::tickets::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ticket.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
