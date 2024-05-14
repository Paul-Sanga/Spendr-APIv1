use axum::{extract::State, Extension, Json};
use sea_orm::DatabaseConnection;

use crate::{
    queries::tip_queries::create_tip_query,
    utilities::{app_error::AppError, MessageResponse},
};

use super::TipData;

#[axum::debug_handler]
pub async fn create_tip(
    State(db): State<DatabaseConnection>,
    Extension(user_id): Extension<i32>,
    Json(tip_data): Json<TipData>,
) -> Result<Json<MessageResponse>, AppError> {
    let _tip_model = create_tip_query(&db, user_id, tip_data).await?;
    let response_message = MessageResponse {
        message: "Tip created successfully".to_owned(),
    };
    Ok(Json(response_message))
}
