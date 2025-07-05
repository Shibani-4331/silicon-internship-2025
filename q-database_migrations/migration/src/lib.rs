pub use sea_orm_migration::prelude::*;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250705_022130_create_users_table::Migration),
            Box::new(m20250705_023505_create_products_table::Migration),
            Box::new(m20250705_023514_create_orders_table::Migration),
            Box::new(m20250705_023520_create_order_items_table::Migration),
        ]
    }
}
mod m20250705_022130_create_users_table;
mod m20250705_023505_create_products_table;
mod m20250705_023514_create_orders_table;
mod m20250705_023520_create_order_items_table;
