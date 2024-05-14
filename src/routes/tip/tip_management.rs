use axum::{
    extract::{Path, State},
    Extension, Json,
};
use sea_orm::DatabaseConnection;

use crate::{
    queries::tip_queries::{create_tip_query, get_tip_by_id_query},
    utilities::{app_error::AppError, MessageResponse},
};

use super::{TipData, TipResponseData};

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

pub async fn get_tip_by_id(
    State(db): State<DatabaseConnection>,
    Extension(user_id): Extension<i32>,
    Path(tip_id): Path<i32>,
) -> Result<Json<(MessageResponse, TipResponseData)>, AppError> {
    let tip_model = get_tip_by_id_query(&db, user_id, tip_id).await?;
    let response_data = TipResponseData {
        tip_message: tip_model.tip_message,
        created_at: tip_model.created_at,
    };
    let response_message = MessageResponse {
        message: "Fetch successful".to_owned(),
    };
    Ok(Json((response_message, response_data)))
}
