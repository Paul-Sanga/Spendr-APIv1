#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub mod authectication;
pub mod user_management;

#[derive(Deserialize)]
pub struct RequestUserData {
    email: String,
    password: String,
    last_name: String,
    first_name: String,
}

#[derive(Deserialize)]
pub struct UpdateUserData {
    pub email: String,
    pub last_name: String,
    pub first_name: String,
}

#[derive(Serialize)]
pub struct MessageResponse {
    message: String,
}

#[derive(Serialize)]
pub struct AuthenticationResponseData {
    token: String,
}

#[derive(Deserialize)]
pub struct RequestUserLoginCred {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUserData {
    email: String,
    last_name: String,
    first_name: String,
}
