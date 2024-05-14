use super::{AuthenticationResponseData, RequestUserData, RequestUserLoginCred};
use crate::{
    queries::user_queries::{create_user_query, login_user, restore_user_account_query},
    utilities::{app_error::AppError, MessageResponse},
};
use axum::{extract::State, Json};
use sea_orm::DatabaseConnection;

pub async fn register_user(
    State(db): State<DatabaseConnection>,
    Json(user_data): Json<RequestUserData>,
) -> Result<Json<MessageResponse>, AppError> {
    let _user_model = create_user_query(&db, user_data).await?;
    let response_message = MessageResponse {
        message: "User Created".to_owned(),
    };
    Ok(Json(response_message))
}

pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(user_data): Json<RequestUserLoginCred>,
) -> Result<Json<(MessageResponse, AuthenticationResponseData)>, AppError> {
    let auth_response = login_user(&db, user_data).await?;
    let response_message = MessageResponse {
        message: "Login successful".to_owned(),
    };
    Ok(Json((response_message, auth_response)))
}

pub async fn restore_user_account(
    State(db): State<DatabaseConnection>,
    Json(user_data): Json<RequestUserLoginCred>,
) -> Result<Json<MessageResponse>, AppError> {
    let _ = restore_user_account_query(&db, user_data).await?;
    let response_message = MessageResponse {
        message: "User account restored".to_owned(),
    };
    Ok(Json(response_message))
}
