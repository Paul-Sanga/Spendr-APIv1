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
        .map_err(|_error| {
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
    let users = Users::find().all(db).await.map_err(|_error| {
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_owned(),
        )
    })?;

    Ok(users)
}
