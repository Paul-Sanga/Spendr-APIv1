use super::app_error::AppError;
use axum::http::StatusCode;
use bcrypt::{ DEFAULT_COST, hash, verify };

pub fn hash_password(password: String)-> Result<String, AppError>{
    hash(password, DEFAULT_COST)
    .map_err(|error|{
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    }) 
}

pub fn verify_password(password: String, hashed_password: String) -> Result<bool, AppError>{
    verify(password, &hashed_password)
    .map_err(|error|{
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    })
}