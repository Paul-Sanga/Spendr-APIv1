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

    let user_existence = check_user_existence(&db, &request_user.email).await?;

    match user_existence.0 {
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
                Ok(Json(ResponseUserData { token }))
            } else {
                return Err(AppError::new(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_owned(),
                ));
            }
        }
    }
}

pub async fn login(
    State(db): State<DatabaseConnection>,
    Json(login_creds): Json<RequestUserLoginCred>,
) -> Result<Json<ResponseUserData>, AppError> {
    let use_exitence = check_user_existence(&db, &login_creds.email).await?;
    match &use_exitence.0 {
        true => {
            if let Some(user_model) = use_exitence.1 {
                if let Ok(password_check) =
                    verify_password(login_creds.password, user_model.password)
                {
                    return match password_check {
                        false => Err(AppError::new(
                            StatusCode::BAD_REQUEST,
                            "invalid login credentials".to_owned(),
                        )),
                        true => {
                            let token = create_token(&login_creds.email)?;
                            Ok(Json(ResponseUserData { token }))
                        }
                    };
                }else{
                    Err(AppError::new(
                        StatusCode::BAD_REQUEST,
                        "invalid login credentials".to_owned(),
                    ))
                }
            }else{
                Err(AppError::new(
                    StatusCode::BAD_REQUEST,
                    "invalid login credentials".to_owned(),
                ))
            }
        }
        false => Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "invalid login credentials".to_owned(),
        )),
    }
}
