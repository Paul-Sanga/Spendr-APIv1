use axum::{
    extract::{Path, State},
    Extension, Json,
};
use sea_orm::{DatabaseConnection, TryIntoModel};

use crate::{
    queries::transactions_queries::{
        create_transaction_query, delete_transaction_query, get_transaction_by_id_query,
        get_transactions_query, update_transaction_query,
    },
    utilities::{app_error::AppError, MessageResponse},
};

use super::{ResponseTransactionData, TransactionData, TransactionUpdateData};

#[axum::debug_handler]
pub async fn create_transaction(
    State(db): State<DatabaseConnection>,
    Extension(user_id): Extension<i32>,
    Json(transactions_data): Json<TransactionData>,
) -> Result<Json<(MessageResponse, ResponseTransactionData)>, AppError> {
    let transactions_model = create_transaction_query(&db, user_id, transactions_data).await?;
    let transactions_model = transactions_model.try_into_model().unwrap();
    let response_transaction = ResponseTransactionData {
        id: transactions_model.id,
        balance: transactions_model.balance,
        category: transactions_model.category,
        update_at: transactions_model.update_at,
    };
    let response_message = MessageResponse {
        message: "Transaction Created".to_owned(),
    };
    Ok(Json((response_message, response_transaction)))
}

#[axum::debug_handler]
pub async fn get_transactions(
    State(db): State<DatabaseConnection>,
    Extension(user_id): Extension<i32>,
) -> Result<Json<(MessageResponse, Vec<ResponseTransactionData>)>, AppError> {
    let transaction_data = get_transactions_query(&db, user_id).await?;
    let mut data: Vec<ResponseTransactionData> = vec![];
    for entry in transaction_data {
        data.push(ResponseTransactionData {
            id: entry.id,
            balance: entry.balance,
            category: entry.category,
            update_at: entry.update_at,
        })
    }
    let respose_message = MessageResponse {
        message: "Fetch Suceessful".to_owned(),
    };
    Ok(Json((respose_message, data)))
}

#[axum::debug_handler]
pub async fn get_transaction_by_id(
    State(db): State<DatabaseConnection>,
    Extension(user_id): Extension<i32>,
    Path(transaction_id): Path<i32>,
) -> Result<Json<(MessageResponse, ResponseTransactionData)>, AppError> {
    let transaction_data = get_transaction_by_id_query(&db, user_id, transaction_id).await?;
    let response_message = MessageResponse {
        message: "Fetch Successful".to_owned(),
    };
    let transaction_response = ResponseTransactionData {
        id: transaction_data.id,
        balance: transaction_data.balance,
        category: transaction_data.category,
        update_at: transaction_data.update_at,
    };
    Ok(Json((response_message, transaction_response)))
}

pub async fn update_transaction(
    State(db): State<DatabaseConnection>,
    Extension(user_id): Extension<i32>,
    Path(transaction_id): Path<i32>,
    Json(transaction_data): Json<TransactionUpdateData>,
) -> Result<Json<MessageResponse>, AppError> {
    let _transaction_data =
        update_transaction_query(&db, transaction_id, user_id, transaction_data).await?;
    let response_message = MessageResponse {
        message: "Update Successful".to_owned(),
    };
    Ok(Json(response_message))
}

pub async fn delete_transaction(
    State(db): State<DatabaseConnection>,
    Extension(user_id): Extension<i32>,
    Path(transaction_id): Path<i32>,
) -> Result<Json<MessageResponse>, AppError> {
    let _deleted_transaction = delete_transaction_query(&db, user_id, transaction_id).await?;
    let response_message = MessageResponse {
        message: "Delete Successful".to_owned(),
    };
    Ok(Json(response_message))
}
