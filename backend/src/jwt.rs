use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use std::env;

use crate::models::{Claims, DbUser};

fn jwt_secret_bytes() -> Vec<u8> {
    env::var("JWT_SECRET")
        .unwrap_or_else(|_| "default_secret".to_string())
        .into_bytes()
}

pub fn create_jwt(user: &DbUser) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        email: user.email.clone(),
        exp: expiration,
        iat: Utc::now().timestamp() as usize,
    };

    let secret = jwt_secret_bytes();
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(&secret),
    )
}

pub fn verify_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let secret = jwt_secret_bytes();
    let validation = Validation::new(Algorithm::HS256);
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(&secret),
        &validation,
    )?;

    Ok(token_data.claims)
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, bcrypt::BcryptError> {
    bcrypt::verify(password, hash)
}
