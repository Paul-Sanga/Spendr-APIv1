use axum::http::StatusCode;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    database::{
        prelude::Users,
        users::{self, Model},
    },
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

pub async fn get_user_by_id_query(db: &DatabaseConnection, id: i32) -> Result<Model, AppError> {
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
    if let Some(user) = user {
        return Ok(user);
    } else {
        return Err(AppError::new(StatusCode::NOT_FOUND, "invalid user id"));
    }
}
