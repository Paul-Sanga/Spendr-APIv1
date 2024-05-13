use serde::Serialize;

pub mod app_error;
pub mod jwt_utils;
pub mod password_utils;

#[derive(Serialize)]
pub struct MessageResponse {
    pub message: String,
}