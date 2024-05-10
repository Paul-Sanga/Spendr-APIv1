use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sea_orm::DatabaseConnection;

use crate::{
    queries::user_queries::{
        delete_user_query, get_user_by_id_query, get_users_query, update_user_query,
    },
    utilities::app_error::AppError,
};

use super::{MessageResponse, ResponseUserData, UpdateUserData};

pub async fn get_users(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseUserData>>, AppError> {
    let users = get_users_query(&db).await?;
    let mut user_data: Vec<ResponseUserData> = vec![];
    for user in users {
        user_data.push(ResponseUserData {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        })
    }
    Ok(Json(user_data))
}

pub async fn get_user_by_id(
    State(db): State<DatabaseConnection>,
    Path(user_id): Path<i32>,
) -> Result<Json<ResponseUserData>, AppError> {
    let user = get_user_by_id_query(&db, user_id).await?;
    if let Some(user) = user {
        let user_data = ResponseUserData {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        };
        return Ok(Json(user_data));
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "invalid user id"));
    }
}

#[axum::debug_handler]
pub async fn update_user(
    Path(id): Path<i32>,
    State(db): State<DatabaseConnection>,
    Json(user_data): Json<UpdateUserData>,
) -> Result<Json<MessageResponse>, AppError> {
    update_user_query(&db, id, user_data).await?;
    let response = MessageResponse {
        message: "Update Successful".to_owned(),
    };
    Ok(Json(response))
}

#[axum::debug_handler]
pub async fn delete_user(
    State(db): State<DatabaseConnection>,
    Path(id): Path<i32>,
) -> Result<Json<MessageResponse>, AppError> {
    delete_user_query(&db, id).await?;
    let response = MessageResponse {
        message: "Delete Successful".to_owned(),
    };
    Ok(Json(response))
}
