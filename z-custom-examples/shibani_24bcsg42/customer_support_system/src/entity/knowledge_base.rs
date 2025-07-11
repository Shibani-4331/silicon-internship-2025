use sea_orm::entity::prelude::*;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "knowledge_base")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub category: String,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Author,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Author => Entity::belongs_to(super::users::Entity)
                .from(Column::CreatedBy)
                .to(super::users::Column::Id)
                .into(),
        }
    }
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Author.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
