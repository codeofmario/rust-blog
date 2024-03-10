use std::sync::Arc;

use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, encode, EncodingKey, Header};
use uuid::Uuid;

use crate::config::settings::Settings;
use crate::enums::error::Error;
use crate::models::token_claims::TokenClaims;
use crate::repository::token_repository::TokenRepository;

#[async_trait]
pub trait TokenService: Send + Sync {
    async fn get_access_token(&self, user_id: Uuid, token_id: Uuid) -> Result<String, Error>;
    async fn get_refresh_token(&self, user_id: Uuid, token_id: Uuid) -> Result<String, Error>;
    async fn save_access_token(&self, user_id: Uuid, token_id: Uuid, token: String) -> Result<bool, Error>;
    async fn save_refresh_token(&self, user_id: Uuid, token_id: Uuid, token: String) -> Result<bool, Error>;
    async fn delete_access_tokens(&self, user_id: Uuid, token_id: Uuid) -> Result<bool, Error>;
    async fn delete_refresh_tokens(&self, user_id: Uuid, token_id: Uuid) -> Result<bool, Error>;
    fn generate_access_token(&self, user_id: Uuid, token_id: Uuid) -> Result<String, Error>;
    fn generate_refresh_token(&self, user_id: Uuid, token_id: Uuid) -> Result<String, Error>;
}

#[derive(Clone)]
pub struct TokenServiceImpl {
    settings: Arc<Settings>,
    repository: Arc<dyn TokenRepository>,
}

impl TokenServiceImpl {
    pub fn new(settings: Arc<Settings>, repository: Arc<dyn TokenRepository>) -> Self {
        TokenServiceImpl { settings, repository }
    }
}

#[async_trait]
impl TokenService for TokenServiceImpl {
    async fn get_access_token(&self, user_id: Uuid, token_id: Uuid) -> Result<String, Error> {
        let key = format!("{}.{}.at", user_id, token_id);
        match self.repository.get_token(key).await {
            Ok(token) => Ok(token.clone()),
            Err(_) => Err(Error::InternalServerError("Cannot get access token.".to_string())),
        }
    }

    async fn get_refresh_token(&self, user_id: Uuid, token_id: Uuid) -> Result<String, Error> {
        let key = format!("{}.{}.rt", user_id, token_id);
        match self.repository.get_token(key).await {
            Ok(token) => Ok(token.clone()),
            Err(_) => Err(Error::InternalServerError("Cannot get refresh token".to_string())),
        }
    }

    async fn save_access_token(&self, user_id: Uuid, token_id: Uuid, token: String) -> Result<bool, Error> {
        let key = format!("{}.{}.at", user_id, token_id);
        let exp = Duration::hours(1);
        match self.repository.save_token(key, token, exp).await {
            Ok(_) => Ok(true),
            Err(_) => Err(Error::InternalServerError("Cannot save access token.".to_string()))
        }
    }

    async fn save_refresh_token(&self, user_id: Uuid, token_id: Uuid, token: String) -> Result<bool, Error> {
        let key = format!("{}.{}.rt", user_id, token_id);
        let exp = Duration::hours(24 * 7);
        match self.repository.save_token(key, token, exp).await {
            Ok(_) => Ok(true),
            Err(_) => Err(Error::InternalServerError("Cannot save refresh token.".to_string()))
        }
    }

    async fn delete_access_tokens(&self, user_id: Uuid, token_id: Uuid) -> Result<bool, Error> {
        let key = format!("{}.{}.at", user_id, token_id);
        match self.repository.delete_token(key).await {
            Ok(_) => Ok(true),
            Err(_) => Err(Error::InternalServerError("Cannot delete access token.".to_string())),
        }
    }

    async fn delete_refresh_tokens(&self, user_id: Uuid, token_id: Uuid) -> Result<bool, Error> {
        let key = format!("{}.{}.rt", user_id, token_id);
        match self.repository.delete_token(key).await {
            Ok(_) => Ok(true),
            Err(_) => Err(Error::InternalServerError("Cannot delete refresh token.".to_string())),
        }
    }

    fn generate_access_token(&self, user_id: Uuid, token_id: Uuid) -> Result<String, Error> {
        let current_time = Utc::now();
        let expiration = current_time + Duration::hours(1); // 1 hour expiration
        let claims = TokenClaims {
            iss: String::from("rustblog"),
            sub: user_id,
            jti: token_id,
            iat: current_time.timestamp(),
            exp: expiration.timestamp(),
            is_refresh: false,
        };

        let secret = self.settings.access_secret.as_bytes();
        match encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(secret)) {
            Ok(token) => Ok(token),
            Err(_) => Err(Error::InternalServerError("Cannot generate access token.".to_string())),
        }
    }

    fn generate_refresh_token(&self, user_id: Uuid, token_id: Uuid) -> Result<String, Error> {
        let current_time = Utc::now();
        let expiration = current_time + Duration::days(7); // 7 days expiration
        let claims = TokenClaims {
            iss: String::from("rustblog"),
            sub: user_id,
            jti: token_id,
            iat: current_time.timestamp(),
            exp: expiration.timestamp(),
            is_refresh: true,
        };

        let secret = self.settings.refresh_secret.as_bytes();
        match encode(&Header::new(Algorithm::HS256), &claims, &EncodingKey::from_secret(secret)) {
            Ok(token) => Ok(token),
            Err(_) => Err(Error::InternalServerError("Cannot generate refresh token.".to_string())),
        }
    }
}