
mod database;
mod entities;

use chrono::Utc;
use database::establish_connection;
use entities::*;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, DatabaseTransaction, DbErr, TransactionError, TransactionTrait};
use sea_orm::{EntityTrait, IntoActiveModel};
#[tokio::main]
async fn main() -> Result<(), DbErr> {
    
    // Connection

    let db = establish_connection().await?;

    // New User Insertion

    // let new_user = users::ActiveModel {
    //     name: Set("Abhilash".to_string()),
    //     email: Set("abhi@yopmail.com".to_string()),
    //     ..Default::default()
    // }.insert(&db)
    // .await?;

    // println!("Inserted User: {:?}", new_user);

    // let _ = users::ActiveModel {
    //     name: Set("Ranit".to_string()),
    //     email: Set("ranit@yopmail.com".to_string()),
    //     ..Default::default()
    // }.insert(&db)
    // .await?;

    // let _ = users::ActiveModel {
    //     name: Set("Ashish".to_string()),
    //     email: Set("ashish@yopmail.com".to_string()),
    //     ..Default::default()
    // }.insert(&db)
    // .await?;

    // // New Product Insertion
    // let new_product = products::ActiveModel {
    //     name: Set("Laptop".to_string()),
    //     price: Set(1000.0),
    //     stock: Set(450),
    //     ..Default::default()
    // }.insert(&db).await?;

    // println!("Inserted Product: {:?}", new_product);

    let new_order= create_order(&db, 2, vec![(10, 5)]).await;

    match new_order {
        Ok(order) => println!("Order created successfully: {:?}", order),
        Err(e) => eprintln!("Error creating order: {}", e),
    }

    Ok(())
}


async fn create_order(db: &DatabaseConnection, user_id: i32, purchase_order: Vec<(i32, i32)>) -> Result<orders::Model, TransactionError<DbErr>> {
    db.transaction(|txn| {
        Box::pin(async move {
            let order = orders::ActiveModel {
                user_id: Set(user_id),
                created_at: Set(Utc::now().naive_utc()),
                ..Default::default()
            }.insert(txn).await?;

            for (product_id, quantity) in purchase_order {
                let product_data  = products::Entity::find_by_id(product_id).one(txn).await?;
                if product_data.is_none() {
                    return Err(DbErr::Custom(format!("Product ID {} not found", product_id)));
                }

                let product_data = product_data.unwrap();
                if product_data.stock < quantity {
                    return Err(DbErr::Custom(format!("Insufficient stock for product ID: {}", product_id)));
                }

                // product_data.stock = product_data.stock - quantity;
                // let _ = product_data.into_active_model()
                //     .update(txn)
                //     .await?;

                let _ = products::ActiveModel {
                        id: Set(product_data.id),
                        stock: Set(product_data.stock - quantity),
                        ..Default::default()
                    }.update(txn).await?;

                let new_order_item = order_items::ActiveModel {
                    order_id: Set(order.id),
                    product_id: Set(product_id),
                    quantity: Set(quantity),
                    ..Default::default()
                }.insert(txn).await?;
            }

            Ok(order)
        })
    }).await
}