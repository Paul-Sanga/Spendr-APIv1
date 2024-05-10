use axum::http::StatusCode;
use chrono::Duration;

use dotenvy_macro::dotenv;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};

use super::app_error::AppError;

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    exp: usize,
    pub email: String,
    pub id: i32,
}

pub fn create_token(email: &String, id: i32) -> Result<String, AppError> {
    let secret_key = dotenv!("SECRET_KEY")
        .parse::<String>()
        .expect("Secret key must be a strig");
    let email = String::from(email);
    let expires_at = chrono::Utc::now() + Duration::hours(1);
    let exp = expires_at.timestamp() as usize;
    let claims = Claims { exp, email, id };
    let token_header = Header::default();
    let key = EncodingKey::from_secret(secret_key.as_bytes());

    encode(&token_header, &claims, &key).map_err(|error| {
        eprintln!("Error creating token: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}

pub fn validate_token(token: &str) -> Result<TokenData<Claims>, AppError> {
    let secret_key = dotenv!("SECRET_KEY")
        .parse::<String>()
        .expect("Secret key must be a strig");
    let key = DecodingKey::from_secret(secret_key.as_bytes());
    let validation = Validation::new(jsonwebtoken::Algorithm::HS256);
    decode::<Claims>(token, &key, &validation)
        .map_err(|error| match error.kind() {
            jsonwebtoken::errors::ErrorKind::InvalidToken
            | jsonwebtoken::errors::ErrorKind::InvalidSignature
            | jsonwebtoken::errors::ErrorKind::ExpiredSignature => {
                AppError::new(StatusCode::UNAUTHORIZED, "invalid token".to_owned())
            }
            _ => AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "error validating token"),
        })
        .map(|claim| claim)
}
