#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub mod authectication;
pub mod user_management;

#[derive(Deserialize)]
pub struct RequestUserData {
    pub email: String,
    pub password: String,
    pub last_name: String,
    pub first_name: String,
}

#[derive(Deserialize, Serialize)]
pub struct UpdateUserData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
}

#[derive(Serialize)]
pub struct AuthenticationResponseData {
    pub email: String,
    pub token: String,
    pub last_name: String,
    pub first_name: String,
}

#[derive(Deserialize)]
pub struct RequestUserLoginCred {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct ResponseUserData {
    pub id: i32,
    pub email: String,
    pub last_name: String,
    pub first_name: String,
}
