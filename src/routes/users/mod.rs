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
