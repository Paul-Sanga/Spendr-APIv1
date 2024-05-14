use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

use crate::{database::tip, routes::tip::TipData, utilities::app_error::AppError};

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
