use axum::{
    extract::{Path, State},
    Json,
};
use sea_orm::DatabaseConnection;

use crate::{
    queries::user_queries::{get_user_by_id_query, get_users_query},
    utilities::app_error::AppError,
};

use super::ResponseUserData;

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
    let user_data = ResponseUserData {
        first_name: user.first_name,
        last_name: user.last_name,
        email: user.email,
    };
    Ok(Json(user_data))
}
