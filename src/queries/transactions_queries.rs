use crate::database::prelude::Transactions;
use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::{
    database::transactions, routes::transactions::TransactionData, utilities::app_error::AppError,
};

pub async fn create_transaction_query(
    db: &DatabaseConnection,
    user_id: i32,
    transactions_data: TransactionData,
) -> Result<transactions::ActiveModel, AppError> {
    let transaction_model = transactions::ActiveModel {
        user_id: Set(user_id),
        balance: Set(transactions_data.balance),
        category: Set(transactions_data.category),
        update_at: Set(chrono::Utc::now().naive_utc().into()),
        ..Default::default()
    };
    let transaction_model = transaction_model.save(db).await.map_err(|error| {
        eprintln!("\x1b[31m error creating transaction: {:?} \x1b[0m", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    })?;
    Ok(transaction_model)
}

pub async fn get_transactions_query(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Vec<transactions::Model>, AppError> {
    let transaction_data = Transactions::find()
        .filter(transactions::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(|error| {
            eprintln!("\x1b[31m error geting transaction: {:?} \x1b[0m", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        })?;
    Ok(transaction_data)
}
