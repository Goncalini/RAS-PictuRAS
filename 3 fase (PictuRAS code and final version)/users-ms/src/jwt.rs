use crate::error::AppError::InvalidToken;
use crate::error::AppResult;
use crate::user::User;
use crate::AppState;
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct AccessTokenClaims {
    pub sub: Uuid,
    pub name: String,
    pub email: String,
    pub token_id: Uuid,
    pub exp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenClaims {
    pub sub: Uuid,
    pub token_id: Uuid,
    pub exp: i64,
}

pub fn create_access_token(state: &AppState, token_id: Uuid, user: &User) -> AppResult<String> {
    let age = state.config.access_token_max_age;
    let expiration = Utc::now().add(age).timestamp();

    let claims = AccessTokenClaims {
        sub: user.uuid,
        name: user.name.clone(),
        email: user.email.clone(),
        token_id,
        exp: expiration,
    };

    let key = &state.config.access_token_private_key;
    let token = jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, key)?;
    Ok(token)
}

pub fn create_refresh_token(state: &AppState, token_id: Uuid, user: &User) -> AppResult<String> {
    let age = state.config.refresh_token_max_age;
    let expiration = Utc::now().add(age).timestamp();

    let claims = RefreshTokenClaims {
        sub: user.uuid,
        token_id,
        exp: expiration,
    };

    let key = &state.config.refresh_token_private_key;
    let token = jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, key)?;
    Ok(token)
}

fn decode_token<T: serde::de::DeserializeOwned>(token: &str, key: &DecodingKey) -> AppResult<T> {
    let token = jsonwebtoken::decode::<T>(token, key, &Validation::new(Algorithm::RS256))?;
    Ok(token.claims)
}

pub fn decode_access_token(state: &AppState, token: &str) -> AppResult<AccessTokenClaims> {
    let token: AccessTokenClaims = decode_token(token, &state.config.access_token_public_key)?;
    validate_expiration_date(token.exp)?;
    Ok(token)
}

pub fn decode_refresh_token(state: &AppState, token: &str) -> AppResult<RefreshTokenClaims> {
    let token: RefreshTokenClaims = decode_token(token, &state.config.refresh_token_public_key)?;
    validate_expiration_date(token.exp)?;
    Ok(token)
}

fn validate_expiration_date(expiration: i64) -> AppResult<()> {
    let now = Utc::now().timestamp();
    if now > expiration {
        return Err(InvalidToken);
    }
    Ok(())
}
