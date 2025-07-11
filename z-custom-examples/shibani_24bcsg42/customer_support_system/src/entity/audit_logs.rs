use sea_orm::entity::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::net::Ipv4Addr;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "audit_logs")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub entity: String,
    pub entity_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub ip_address: String,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::User => Entity::belongs_to(super::users::Entity)
                .from(Column::UserId)
                .to(super::users::Column::Id)
                .into(),
        }
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
