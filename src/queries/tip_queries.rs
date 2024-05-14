use axum::{http::StatusCode, Json};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, IntoActiveModel,
    QueryFilter, Set,
};

use crate::{
    database::{prelude::Tip, tip},
    routes::tip::{TipData, TipUpdateData},
    utilities::app_error::AppError,
};

pub async fn create_tip_query(
    db: &DatabaseConnection,
    user_id: i32,
    tip_data: TipData,
) -> Result<tip::ActiveModel, AppError> {
    let tip_model = tip::ActiveModel {
        tip_message: Set(tip_data.tip_message),
        user_id: Set(user_id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };
    if let Ok(tip_model) = tip_model.save(db).await {
        return Ok(tip_model);
    } else {
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error",
        ));
    }
}

pub async fn get_tip_by_id_query(
    db: &DatabaseConnection,
    user_id: i32,
    tip_id: i32,
) -> Result<tip::Model, AppError> {
    let tip_model = Tip::find()
        .filter(
            Condition::all()
                .add(tip::Column::Id.eq(tip_id))
                .add(tip::Column::UserId.eq(user_id)),
        )
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("\x1b[31m error getting tip by id: {:?} \x1b[0m", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        })?;

    if let Some(tip_model) = tip_model {
        return Ok(tip_model);
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "invalid tip id"));
    }
}

pub async fn get_tips_query(
    db: &DatabaseConnection,
    user_id: i32,
) -> Result<Vec<tip::Model>, AppError> {
    let tip_model = Tip::find()
        .filter(tip::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(|error| {
            eprintln!("\x1b[31m error geting tips: {:?} \x1b[om", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        })?;
    Ok(tip_model)
}

pub async fn update_tip_query(
    db: &DatabaseConnection,
    user_id: i32,
    tip_id: i32,
    tip_data: TipUpdateData,
) -> Result<tip::ActiveModel, AppError> {
    let mut tip_model = get_tip_by_id_query(db, user_id, tip_id)
        .await?
        .into_active_model();
    if let Some(tip_message) = tip_data.tip_message {
        tip_model.tip_message = Set(tip_message);
    }
    if let Ok(tip_model) = tip_model.save(db).await {
        return Ok(tip_model);
    } else {
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error",
        ));
    }
}
