use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::{
    database::{
        budget::{self, Model},
        prelude::Budget,
    },
    routes::budget::RequestBudgetData,
    utilities::app_error::AppError,
};

pub async fn create_budget_query(
    db: &DatabaseConnection,
    user_id: i32,
    budget_data: RequestBudgetData,
) -> Result<budget::ActiveModel, AppError> {
    let budget_model = budget::ActiveModel {
        user_id: Set(user_id),
        category: Set(budget_data.category),
        amount_budgeted: Set(budget_data.amount_budgeted),
        amount_spent: Set(budget_data.amount_spent),
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

pub async fn get_budget_query(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Vec<Model>, AppError> {
    let budget = Budget::find()
        .filter(budget::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(|error| {
            eprintln!("\x1b[31m error fetching user budget {:?}\x1b[0m", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server")
        })?;
    Ok(budget)
}

pub async fn update_budget_query(
    db: &DatabaseConnection,
    user_id: i32,
    budget_id: i32,
) -> Result<(), AppError> {
    let budget_model = Budget::find()
        .filter(budget::Column::Id.eq(budget_id))
        .one(db)
        .await
        .map_err(|error| {
            eprint!("\x1b[31m error geting budget by id: {:?} \x1b[0m", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error");
        });
    if let Ok(budget_model) = budget_model {
        let budget_model: budget::ActiveModel = budget_model.unwrap().into();
    } else {
        AppError::new(StatusCode::NOT_FOUND, "invalid budget id");
    }

    todo!();
}
