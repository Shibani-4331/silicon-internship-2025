use sea_orm::entity::prelude::*;
use uuid::Uuid;
use chrono::NaiveDate;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "analytics")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub date: NaiveDate,
    pub total_tickets: i32,
    pub resolved_tickets: i32,
    pub avg_response_time: i64,
    pub agent_id: Uuid,
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::AgentId",
        to = "super::users::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    Agent,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Agent.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
