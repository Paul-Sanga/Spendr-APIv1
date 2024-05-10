use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use sea_orm::DatabaseConnection;

use crate::{
    queries::user_queries::check_user_existence,
    utilities::{app_error::AppError, jwt_utils::validate_token},
};

pub async fn require_authentication(
    State(db): State<DatabaseConnection>,
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    let header_token = if let Some(token) = headers.get("x-auth-token") {
        token.to_str().map_err(|error| {
            eprintln!(
                "\x1b[31m Error extracting token from headers: {:?} \x1b[om",
                error
            );
            AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "error reading token")
        })?
    } else {
        return Err(AppError::new(StatusCode::UNAUTHORIZED, "not authenticated"));
    };

    let token_data = validate_token(header_token)?;
    let (user_exitence, user_model) = check_user_existence(&db, &token_data.claims.email).await?;
    if user_exitence {
        if let Some(user_model) = user_model {
            request.extensions_mut().insert(user_model.email);
            request.extensions_mut().insert(token_data.claims.id);
            return Ok(next.run(request).await);
        } else {
            return Err(AppError::new(
                StatusCode::UNAUTHORIZED,
                "Invalid token".to_owned(),
            ));
        }
    } else {
        return Err(AppError::new(
            StatusCode::UNAUTHORIZED,
            "Invalid token".to_owned(),
        ));
    }
}
