use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, DeleteResult, EntityTrait,
    IntoActiveModel, ModelTrait, QueryFilter, Set,
};

use crate::{
    database::{
        budget::{self, Model},
        prelude::Budget,
    },
    routes::budget::{RequestBudgetData, RequestUpdateBudgetData},
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

pub async fn get_budget_by_id_query(
    db: &DatabaseConnection,
    budget_id: i32,
    user_id: i32,
) -> Result<budget::Model, AppError> {
    let budget_model = Budget::find()
        .filter(
            Condition::all()
                .add(budget::Column::Id.eq(budget_id))
                .add(budget::Column::UserId.eq(user_id)),
        )
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("\x1b[31m error getting budget by id: {:?} \x1b[0m", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        })?;
    if let Some(budget_model) = budget_model {
        return Ok(budget_model);
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "invalid budget id"));
    }
}

pub async fn update_budget_query(
    db: &DatabaseConnection,
    user_id: i32,
    budget_id: i32,
    budget_data: RequestUpdateBudgetData,
) -> Result<budget::ActiveModel, AppError> {
    let budget_model = get_budget_by_id_query(db, budget_id, user_id).await?;
    let mut budget_model = budget_model.into_active_model();
    if let Some(category) = budget_data.category {
        budget_model.category = Set(category);
    }
    if let Some(amount_budgeted) = budget_data.amount_budgeted {
        budget_model.amount_budgeted = Set(amount_budgeted);
    }
    if let Some(amount_spent) = budget_data.amount_spent {
        budget_model.amount_spent = Set(amount_spent);
    }
    budget_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
    return budget_model.save(db).await.map_err(|error| {
        eprintln!("\x1b[31m error updating budget: {:?} \x1b[0m", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
    });
}

pub async fn delete_budget_query(
    db: &DatabaseConnection,
    budget_id: i32,
    user_id: i32,
) -> Result<DeleteResult, AppError> {
    let budget_model = get_budget_by_id_query(db, budget_id, user_id).await?;
    if let Ok(budget_model) = budget_model.delete(db).await {
        return Ok(budget_model);
    } else {
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error",
        ));
    }
}
