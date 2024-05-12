use super::{AuthenticationResponseData, RequestUserData, RequestUserLoginCred};
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
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};

pub async fn register_user(
    State(db): State<DatabaseConnection>,
    Json(request_user): Json<RequestUserData>,
) -> Result<Json<AuthenticationResponseData>, AppError> {
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
        new_user.first_name = Set(request_user.first_name);
        new_user.last_name = Set(request_user.last_name);
        new_user.email = Set(request_user.email);
        new_user.password = Set(hash_password(request_user.password)?);

        if let Ok(user_model) = new_user.save(&db).await {
            let user_model = user_model.try_into_model().unwrap();
            let token = create_token(&user_model.email, user_model.id)?;
            Ok(Json(AuthenticationResponseData { token }))
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
) -> Result<Json<AuthenticationResponseData>, AppError> {
    let (use_exitence, user_model) = check_user_existence(&db, &login_creds.email).await?;

    if !use_exitence {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "invalid login credentials".to_owned(),
        ));
    }

    let mut _id: i32 = 0;
    let mut _valid_password = false;
    if let Some(user_model) = user_model {
        _id = user_model.id;
        _valid_password = verify_password(login_creds.password, user_model.password)?;
    } else {
        return Err(AppError::new(StatusCode::CONFLICT, "Account deleted"));
    };

    if !_valid_password {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "invalid login credentials".to_owned(),
        ));
    } else {
        let token = create_token(&login_creds.email, _id)?;
        return Ok(Json(AuthenticationResponseData { token }));
    }
}
