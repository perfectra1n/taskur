use crate::errors::{AppError, AppResult};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: i64,    // expiration time
    pub iat: i64,    // issued at
}

impl Claims {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        let exp = (now + Duration::days(7)).timestamp();

        Self {
            sub: user_id.to_string(),
            exp,
            iat: now.timestamp(),
        }
    }

    pub fn user_id(&self) -> Result<Uuid, uuid::Error> {
        Uuid::parse_str(&self.sub)
    }
}

pub fn create_token(user_id: Uuid, secret: &str) -> AppResult<String> {
    let claims = Claims::new(user_id);
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}

pub fn verify_token(token: &str, secret: &str) -> AppResult<Claims> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
