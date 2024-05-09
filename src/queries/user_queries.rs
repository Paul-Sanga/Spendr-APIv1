use axum::http::StatusCode;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};

use crate::{
    database::{
        prelude::Users,
        users::{self, Model},
    },
    routes::users::UpdateUserData,
    utilities::app_error::AppError,
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
    let current_user_data = get_user_by_id_query(db, id).await?;
    let current_user_data = if let Some(current_user_data) = current_user_data {
        current_user_data
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "invalid user id"));
    };

    let user_model = get_user_by_id_query(db, id).await?;
    let mut user_model: users::ActiveModel = user_model.unwrap().into();
    user_model.id = Set(current_user_data.id);
    user_model.email = Set(user_data.email);
    user_model.last_name = Set(user_data.last_name);
    user_model.first_name = Set(user_data.first_name);
    user_model.password = Set(current_user_data.password);
    user_model.is_verified = Set(current_user_data.is_verified);
    user_model.is_deleted = Set(current_user_data.is_deleted);
    user_model.created_at = Set(current_user_data.created_at);
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
