use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, IntoActiveModel, QueryFilter,
    Set,
};

use crate::{
    database::{
        prelude::Users,
        users::{self, Model},
    },
    routes::users::{
        AuthenticationResponseData, RequestUserData, RequestUserLoginCred, UpdateUserData,
    },
    utilities::{
        app_error::AppError,
        jwt_utils::create_token,
        password_utils::{hash_password, verify_password},
    },
};

pub async fn check_user_existence(
    db: &DatabaseConnection,
    email: &String,
) -> Result<(bool, Option<Model>), AppError> {
    let user = Users::find()
        .filter(users::Column::Email.eq(email))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("\x1b[31m Error geting user by id {:?} \x1b[0m", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_owned(),
            )
        })?;

    if let Some(user) = user {
        if !user.is_deleted {
            return Ok((true, Some(user)));
        } else {
            return Ok((true, None));
        }
    } else {
        return Ok((false, None));
    }
}

pub async fn create_user_query(
    db: &DatabaseConnection,
    user_data: RequestUserData,
) -> Result<users::ActiveModel, AppError> {
    let (user_existence, _) = check_user_existence(db, &user_data.email).await?;
    if user_existence {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "This email is already registered",
        ));
    } else {
        let user_model = users::ActiveModel {
            first_name: Set(user_data.first_name),
            last_name: Set(user_data.last_name),
            email: Set(user_data.email),
            password: Set(hash_password(user_data.password)?),
            ..Default::default()
        };
        if let Ok(user_model) = user_model.save(db).await {
            return Ok(user_model);
        } else {
            return Err(AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal server error",
            ));
        }
    }
}

pub async fn login_user(
    db: &DatabaseConnection,
    user_data: RequestUserLoginCred,
) -> Result<AuthenticationResponseData, AppError> {
    let (user_exitence, user_model) = check_user_existence(db, &user_data.email).await?;

    if !user_exitence {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "invalid login credentials",
        ));
    }

    if let Some(user_model) = user_model {
        if !verify_password(user_data.password, user_model.password)? {
            return Err(AppError::new(
                StatusCode::BAD_REQUEST,
                "invalid login credentials",
            ));
        } else {
            let token = create_token(&user_model.email, user_model.id)?;
            let auth_response = AuthenticationResponseData {
                email: user_model.email,
                token,
                last_name: user_model.last_name,
                first_name: user_model.first_name,
            };
            return Ok(auth_response);
        }
    } else {
        return Err(AppError::new(
            StatusCode::CONFLICT,
            "user account is deleted",
        ));
    }
}

pub async fn restore_user_account_query(
    db: &DatabaseConnection,
    user_data: RequestUserLoginCred,
) -> Result<users::ActiveModel, AppError> {
    let user_model = Users::find()
        .filter(users::Column::Email.eq(user_data.email))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("\x1b[31m error restoring user account: {:?} \x1b[0m", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        })?;

    if let Some(user_model) = user_model {
        if user_model.is_deleted {
            let mut user_model = user_model.into_active_model();
            user_model.is_deleted = Set(false);
            return match user_model.save(db).await {
                Ok(user_model) => Ok(user_model),
                Err(_error) => {
                    return Err(AppError::new(
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "internal server error",
                    ));
                }
            };
        } else {
            return Err(AppError::new(
                StatusCode::CONFLICT,
                "account is already active",
            ));
        }
    } else {
        return Err(AppError::new(
            StatusCode::BAD_REQUEST,
            "invalid email address",
        ));
    }
}

pub async fn get_users_query(db: &DatabaseConnection) -> Result<Vec<Model>, AppError> {
    let users = Users::find().all(db).await.map_err(|error| {
        eprintln!("\x1b[31m Error geting user by id {:?} \x1b[0m", error);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_owned(),
        )
    })?;

    Ok(users)
}

pub async fn get_user_by_id_query(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<Model>, AppError> {
    let user = Users::find()
        .filter(users::Column::Id.eq(id))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("\x1b[31m Error geting user by id {:?} \x1b[0m", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "error getting user by id".to_owned(),
            )
        })?;

    Ok(user)
}

pub async fn update_user_query(
    db: &DatabaseConnection,
    id: i32,
    user_data: UpdateUserData,
) -> Result<users::ActiveModel, AppError> {
    let user_model = get_user_by_id_query(db, id).await?;
    let mut user_model: users::ActiveModel = user_model.unwrap().into();
    if let Some(email) = user_data.email {
        user_model.email = Set(email)
    }
    if let Some(last_name) = user_data.last_name {
        user_model.last_name = Set(last_name);
    }
    if let Some(first_name) = user_data.first_name {
        user_model.first_name = Set(first_name);
    }
    user_model.updated_at = Set(Some(chrono::Utc::now().naive_utc()));
    let updated_user = user_model.save(db).await;
    if let Ok(updated_user) = updated_user {
        return Ok(updated_user);
    } else {
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error",
        ));
    }
}

pub async fn delete_user_query(db: &DatabaseConnection, id: i32) -> Result<(), AppError> {
    let user = Users::find()
        .filter(users::Column::Id.eq(id))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("\x1b[31m Error fetching user by id: {:?} \x1b[0m", error);
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
        })?;

    let mut user_model: users::ActiveModel = user.unwrap().into();
    user_model.is_deleted = Set(true);
    if let Ok(_saved_user) = user_model.save(db).await {
        return Ok(());
    } else {
        return Err(AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal server error",
        ));
    }
}
