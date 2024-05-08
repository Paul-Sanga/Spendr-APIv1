use super::{RequestUserData, RequestUserLoginCred, ResponseUserData};
use crate::{
    database::users,
    queries::user_queries::check_user_existence,
    utilities::{
        app_error::AppError,
        jwt_utils::create_token,
        password_utils::{hash_password, verify_password},
    },
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

    let (user_existence, _) = check_user_existence(&db, &request_user.email).await?;

    if user_existence {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "User email is already registered".to_owned(),
        ));
    } else {
        let token = create_token(&request_user.email)?;
        new_user.first_name = Set(request_user.first_name);
        new_user.last_name = Set(request_user.last_name);
        new_user.email = Set(request_user.email);
        new_user.password = Set(hash_password(request_user.password)?);

        if let Ok(_) = new_user.save(&db).await {
            Ok(Json(ResponseUserData { token }))
        } else {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_owned(),
            ));
        }
    }
}

pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(login_creds): Json<RequestUserLoginCred>,
) -> Result<Json<ResponseUserData>, AppError> {
    let (use_exitence, user_model) = check_user_existence(&db, &login_creds.email).await?;

    if !use_exitence {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "invalid login credentials".to_owned(),
        ));
    }

    let valid_password = if let Some(user_model) = user_model {
        verify_password(login_creds.password, user_model.password)?
    } else {
        false
    };

    if !valid_password {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "invalid login credentials".to_owned(),
        ));
    } else {
        let token = create_token(&login_creds.email)?;
        return Ok(Json(ResponseUserData { token }));
    }
}
