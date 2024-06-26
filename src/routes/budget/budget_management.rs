use axum::{
    extract::{Path, State},
    http::StatusCode,
    Extension, Json,
};
use sea_orm::{DatabaseConnection, TryIntoModel};

use crate::{
    queries::budget_queries::{
        create_budget_query, delete_budget_query, get_budget_by_id_query, get_budget_query,
        update_budget_query,
    },
    utilities::{app_error::AppError, MessageResponse},
};

use super::{RequestBudgetData, RequestUpdateBudgetData, ResponseBudgetData};

#[axum::debug_handler]
pub async fn create_budget(
    State(db): State<DatabaseConnection>,
    Extension(user_id): Extension<i32>,
    Json(budget_data): Json<RequestBudgetData>,
) -> Result<Json<(MessageResponse, ResponseBudgetData)>, AppError> {
    let saved_budget = create_budget_query(&db, user_id, budget_data).await?;
    let saved_budget = if let Ok(saved_budget) = saved_budget.try_into_model() {
        saved_budget
    } else {
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error",
        ));
    };
    let response = ResponseBudgetData {
        id: saved_budget.id,
        category: saved_budget.category,
        amount_spent: saved_budget.amount_spent,
        amount_budgeted: saved_budget.amount_budgeted,
        created_at: saved_budget.created_at,
    };
    let response_message = MessageResponse {
        message: "Budget created".to_owned(),
    };
    Ok(Json((response_message, response)))
}

#[axum::debug_handler]
pub async fn get_budget(
    State(db): State<DatabaseConnection>,
    Extension(user_id): Extension<i32>,
) -> Result<Json<(MessageResponse, Vec<ResponseBudgetData>)>, AppError> {
    let budget_model = get_budget_query(&db, user_id).await?;

    let response_budget: Vec<ResponseBudgetData> = budget_model
        .into_iter()
        .map(|entry| ResponseBudgetData {
            id: entry.id,
            category: entry.category,
            amount_budgeted: entry.amount_budgeted,
            amount_spent: entry.amount_spent,
            created_at: entry.created_at,
        })
        .collect();
    let response_message = MessageResponse {
        message: "Fetch Successful".to_owned(),
    };
    Ok(Json((response_message, response_budget)))
}

#[axum::debug_handler]
pub async fn get_budget_by_id(
    State(db): State<DatabaseConnection>,
    Path(budget_id): Path<i32>,
    Extension(user_id): Extension<i32>,
) -> Result<Json<(MessageResponse, ResponseBudgetData)>, AppError> {
    println!("\x1b[32m reached here \x1b[0m");
    let budget_model = get_budget_by_id_query(&db, budget_id, user_id).await?;
    let response = ResponseBudgetData {
        id: budget_model.id,
        category: budget_model.category,
        amount_budgeted: budget_model.amount_budgeted,
        amount_spent: budget_model.amount_spent,
        created_at: budget_model.created_at,
    };
    let response_message = MessageResponse {
        message: "Fetch Successful".to_owned(),
    };
    Ok(Json((response_message, response)))
}

#[axum::debug_handler]
pub async fn update_budget(
    State(db): State<DatabaseConnection>,
    Extension(user_id): Extension<i32>,
    Path(budget_id): Path<i32>,
    Json(budget_data): Json<RequestUpdateBudgetData>,
) -> Result<Json<MessageResponse>, AppError> {
    let _budget_model = update_budget_query(&db, user_id, budget_id, budget_data).await?;
    let response = MessageResponse {
        message: "Update successful".to_owned(),
    };
    Ok(Json(response))
}

#[axum::debug_handler]
pub async fn delete_budget(
    State(db): State<DatabaseConnection>,
    Extension(user_id): Extension<i32>,
    Path(budget_id): Path<i32>,
) -> Result<Json<MessageResponse>, AppError> {
    let _deleted_budget = delete_budget_query(&db, budget_id, user_id).await?;
    let response = MessageResponse {
        message: "delete successful".to_owned(),
    };
    Ok(Json(response))
}
