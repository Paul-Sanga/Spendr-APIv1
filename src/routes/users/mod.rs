#![allow(dead_code)]
use serde::{Deserialize, Serialize};

pub mod authectication;
pub mod user_management;

#[derive(Deserialize)]
pub struct RequestUserData {
    email: String,
    first_name: String,
    last_name: String,
    password: String,
}

#[derive(Serialize)]
pub struct ResponseUserData {
    token: String,
}
