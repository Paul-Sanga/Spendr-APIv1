use super::{RequestUserData, ResponseUserData};
use crate::{
    database::users,
    queries::user_queries::check_user_existence,
    utilities::{app_error::AppError, jwt_utils::create_token, password_utils::hash_password},
};
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};

pub async fn register_user(
    State(db): State<DatabaseConnection>,
    Json(request_user): Json<RequestUserData>,
) -> Result<Json<ResponseUserData>, AppError> {
    let mut new_user = users::ActiveModel {
        ..Default::default()
    };

    let user_existence = check_user_existence(&db, &request_user.email).await?;

    match user_existence {
        true => Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "User email is already registered".to_owned(),
        )),
        false => {
            let token = create_token(&request_user.email)?;
            new_user.first_name = Set(request_user.first_name);
            new_user.last_name = Set(request_user.last_name);
            new_user.email = Set(request_user.email);
            new_user.password = Set(hash_password(request_user.password)?);

            if let Ok(_) = new_user.save(&db).await {
                
                Ok(Json(ResponseUserData {
                    token,
                }))
            } else {
                return Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_owned(),
                ));
            }
        }
    }
}
