use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Orders::Table)
                    .if_not_exists()
                    .col(pk_auto(Orders::Id))
                    .col(integer(Orders::UserId).not_null())
                    .col(timestamp(Orders::CreatedAt).not_null())
                    .foreign_key(ForeignKey::create()
                        .name("fk_user_id_orders")
                        .from(Orders::Table, Orders::UserId)
                        .to(Users::Table, Users::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Orders::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Orders {
    Table,
    Id,
    UserId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id
}