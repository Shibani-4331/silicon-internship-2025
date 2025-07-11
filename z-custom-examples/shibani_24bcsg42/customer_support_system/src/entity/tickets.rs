use sea_orm::entity::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tickets")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub channel: String,
    pub customer_id: Uuid,
    pub assigned_agent_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Customer,
    Agent,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Customer => Entity::belongs_to(super::customers::Entity)
                .from(Column::CustomerId)
                .to(super::customers::Column::Id)
                .into(),
            Self::Agent => Entity::belongs_to(super::users::Entity)
                .from(Column::AssignedAgentId)
                .to(super::users::Column::Id)
                .into(),
        }
    }
}

impl Related<super::customers::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Customer.def()
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Agent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
