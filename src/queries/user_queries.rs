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
    let user_model = get_user_by_id_query(db, id).await?;
    let mut user_model: users::ActiveModel = user_model.unwrap().into();
    if let Some(email) = user_data.email {
        user_model.email = Set(email)
    }
    if let Some(last_name) = user_data.last_name{
        user_model.last_name = Set(last_name);
    }
    if let Some(first_name) = user_data.first_name{
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
