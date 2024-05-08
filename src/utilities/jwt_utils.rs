use axum::http::StatusCode;
use chrono::Duration;

use dotenvy_macro::dotenv;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: usize,
    email: String,
}

pub fn create_token(email: &String) -> Result<String, AppError> {
    let secret_key = dotenv!("SECRET_KEY")
        .parse::<String>()
        .expect("Secret key must be a strig");
    let email = String::from(email);
    let expires_at = chrono::Utc::now() + Duration::hours(1);
    let exp = expires_at.timestamp() as usize;
    let claims = Claims { exp, email };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret_key.as_bytes());

    encode(&token_header, &claims, &key).map_err(|error| {
        eprintln!("Error creating token: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
