use sea_orm::entity::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "tags")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub tag_name: String,
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
