use axum::{extract::State, Json};
use sea_orm::DatabaseConnection;

use crate::{queries::user_queries::get_users_query, utilities::app_error::AppError};

use super::ResponseusersData;

pub async fn get_users(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<ResponseusersData>>, AppError> {
    let users = get_users_query(&db).await?;
    let mut user_data: Vec<ResponseusersData> = vec![];
    for user in users {
        user_data.push(ResponseusersData {
            first_name: user.first_name,
            last_name: user.last_name,
            email: user.email,
        })
    }
    Ok(Json(user_data))
}
