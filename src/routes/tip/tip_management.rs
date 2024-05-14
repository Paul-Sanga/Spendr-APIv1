use axum::{
    extract::{Path, State},
    response::Response,
    Extension, Json,
};
use sea_orm::DatabaseConnection;

use crate::{
    database::tip,
    queries::tip_queries::{
        create_tip_query, get_tip_by_id_query, get_tips_query, update_tip_query,
    },
    utilities::{app_error::AppError, MessageResponse},
};

use super::{TipData, TipResponseData, TipUpdateData};

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

#[axum::debug_handler]
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

#[axum::debug_handler]
pub async fn get_tips(
    State(db): State<DatabaseConnection>,
    Extension(user_id): Extension<i32>,
) -> Result<Json<(MessageResponse, Vec<TipResponseData>)>, AppError> {
    let tip_model = get_tips_query(&db, user_id).await?;
    let data: Vec<TipResponseData> = tip_model
        .into_iter()
        .map(|entry| TipResponseData {
            tip_message: entry.tip_message,
            created_at: entry.created_at,
        })
        .collect::<Vec<TipResponseData>>();
    let response_message = MessageResponse {
        message: "Fetch successful".to_owned(),
    };
    Ok(Json((response_message, data)))
}

#[axum::debug_handler]
pub async fn update_tip(
    State(db): State<DatabaseConnection>,
    Extension(user_id): Extension<i32>,
    Path(tip_id): Path<i32>,
    Json(tip_data): Json<TipUpdateData>,
) -> Result<Json<MessageResponse>, AppError> {
    let _tip_model = update_tip_query(&db, user_id, tip_id, tip_data).await?;
    let response_message = MessageResponse {
        message: "Update Successful".to_owned(),
    };
    Ok(Json(response_message))
}
