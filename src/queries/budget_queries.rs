use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::{database::budget, routes::budget::RequestBudgetData, utilities::app_error::AppError};

pub async fn create_budget_query(
    db: &DatabaseConnection,
    user_id: i32,
    budget_data: RequestBudgetData,
) -> Result<budget::ActiveModel, AppError> {
    let budget_model = budget::ActiveModel {
        user_id: Set(user_id),
        category: Set(budget_data.category),
        amount: Set(budget_data.amount),
        ..Default::default()
    };
    if let Ok(saved_budget) = budget_model.save(db).await {
        return Ok(saved_budget);
    } else {
        eprintln!("\x1b[31m error creating budget \x1b[0m");
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error",
        ));
    }
}
