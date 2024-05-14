use crate::{database::prelude::Transactions, routes::transactions::TransactionUpdateData};
use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DeleteResult, EntityTrait,
    IntoActiveModel, ModelTrait, QueryFilter, Set,
};

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

pub async fn get_transaction_by_id_query(
    db: &DatabaseConnection,
    user_id: i32,
    transaction_id: i32,
) -> Result<transactions::Model, AppError> {
    let transaction_data = Transactions::find()
        .filter(
            Condition::all()
                .add(transactions::Column::UserId.eq(user_id))
                .add(transactions::Column::Id.eq(transaction_id)),
        )
        .one(db)
        .await
        .map_err(|error| {
            eprintln!(
                "\x1b[31m error geting trnascation by id: {:?} \x1b[0m",
                error
            );
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        })?;

    if let Some(transaction_data) = transaction_data {
        return Ok(transaction_data);
    } else {
        return Err(AppError::new(
            StatusCode::NOT_FOUND,
            "invalid transaction id",
        ));
    }
}

pub async fn update_transaction_query(
    db: &DatabaseConnection,
    transaction_id: i32,
    user_id: i32,
    transaction_data: TransactionUpdateData,
) -> Result<transactions::ActiveModel, AppError> {
    let mut transaction_model = get_transaction_by_id_query(db, user_id, transaction_id)
        .await?
        .into_active_model();
    if let Some(balance) = transaction_data.balance {
        transaction_model.balance = Set(balance)
    }
    if let Some(category) = transaction_data.category {
        transaction_model.category = Set(category)
    }
    transaction_model.update_at = Set(chrono::Utc::now().naive_utc().into());
    if let Ok(transaction_model) = transaction_model.save(db).await {
        return Ok(transaction_model);
    } else {
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error".to_owned(),
        ));
    }
}

pub async fn delete_transaction_query(
    db: &DatabaseConnection,
    user_id: i32,
    transaction_id: i32,
) -> Result<DeleteResult, AppError> {
    let transaction_model = get_transaction_by_id_query(db, user_id, transaction_id).await?;
    if let Ok(transaction_model) = transaction_model.delete(db).await {
        return Ok(transaction_model);
    } else {
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error",
        ));
    }
}
