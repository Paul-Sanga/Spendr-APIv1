use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::{DatabaseConnection, TryIntoModel};

use crate::{queries::budget_queries::create_budget_query, utilities::app_error::AppError};

use super::{RequestBudgetData, ResponseBudgetData};

#[axum::debug_handler]
pub async fn create_budget(
    State(db): State<DatabaseConnection>,
    Extension(user_id): Extension<i32>,
    Json(budget_data): Json<RequestBudgetData>,
) -> Result<Json<ResponseBudgetData>, AppError> {
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
        category: saved_budget.category,
        amount: saved_budget.amount,
        created_at: saved_budget.created_at,
    };
    Ok(Json(response))
}
