use axum::http::StatusCode;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    database::{prelude::Users, users},
    utilities::app_error::AppError,
};

pub async fn check_user_existence(
    db: &DatabaseConnection,
    email: &String,
) -> Result<bool, AppError> {
    let user = Users::find()
        .filter(users::Column::Email.eq(email))
        .one(db)
        .await
        .map_err(|_error| {
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
        })?;

    if let Some(_user) = user {
        return Ok(true);
    } else {
        return Ok(false);
    }
}
