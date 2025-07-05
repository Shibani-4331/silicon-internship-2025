use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(OrderItems::Table)
                    .if_not_exists()
                    .col(pk_auto(OrderItems::Id))
                    .col(integer(OrderItems::OrderId).not_null())
                    .col(integer(OrderItems::ProductId).not_null())
                    .col(integer(OrderItems::Quantity).not_null())
                    .foreign_key(ForeignKey::create()
                        .name("fk_order_id_order_items")
                        .from(OrderItems::Table, OrderItems::OrderId)
                        .to(Orders::Table, Orders::Id)
                    )
                    .foreign_key(ForeignKey::create()
                        .name("fk_product_id_order_items")
                        .from(OrderItems::Table, OrderItems::ProductId)
                        .to(Products::Table, Products::Id)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrderItems::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum OrderItems {
    Table,
    Id,
    OrderId,
    ProductId,
    Quantity
}

#[derive(DeriveIden)]
enum Orders {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Products {
    Table,
    Id,
}