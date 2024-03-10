use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::config::settings::Settings;
use crate::dtos::response::tokens_response_dto::TokensResponseDto;
use crate::enums::error::Error;
use crate::models::user::User;
use crate::services::token_service::TokenService;
use crate::services::user_service::UserService;
use crate::utils::password_util::check_password_hash;
use crate::utils::token_util::{get_bearer_token, parse_jwt_token};

#[async_trait]
pub trait AuthService: Send + Sync {
  async fn login(&self, user: User) -> Result<TokensResponseDto, Error>;
  async fn logout(&self, auth_header: String) -> Result<(), Error>;
  async fn refresh(&self, token_string: String) -> Result<TokensResponseDto, Error>;
}

pub struct AuthServiceImpl {
  user_service: Arc<dyn UserService>,
  token_service: Arc<dyn TokenService>,
  settings: Arc<Settings>,
}

impl AuthServiceImpl {
  pub fn new(user_service: Arc<dyn UserService>, token_service: Arc<dyn TokenService>, settings: Arc<Settings>) -> Self {
    AuthServiceImpl {
      user_service,
      token_service,
      settings,
    }
  }

  fn check_user(&self, user: &User, db_user: &User) -> Result<(), Error> {
    let email = user.email.clone();
    let password = user.password.clone();
    let hash = db_user.password.clone();

    if db_user.email != *email || !check_password_hash(password, hash) {
      return Err(Error::InternalServerError("Authentication failed.".to_string()));
    }

    return Ok(());
  }

  fn generate_tokens(&self, user_id: &Uuid, token_id: &Uuid) -> Result<TokensResponseDto, Error> {
    let result = self.token_service.generate_access_token(user_id.clone(), token_id.clone());
    if result.is_err() {
      return Err(result.unwrap_err());
    }

    let access_token = result.unwrap();

    let result = self.token_service.generate_refresh_token(user_id.clone(), token_id.clone());
    if result.is_err() {
      return Err(result.unwrap_err());
    }

    let refresh_token = result.unwrap();

    Ok(TokensResponseDto { access_token, refresh_token })
  }

  async fn save_tokens(&self, user_id: &Uuid, token_id: &Uuid, tokens: &TokensResponseDto) -> Result<(), Error> {
    let result = self.token_service.save_access_token(user_id.clone(), token_id.clone(), tokens.access_token.clone()).await;
    if result.is_err() {
      return Err(result.unwrap_err());
    }

    let result = self.token_service.save_refresh_token(user_id.clone(), token_id.clone(), tokens.refresh_token.clone()).await;
    if result.is_err() {
      return Err(result.unwrap_err());
    }

    Ok(())
  }

  async fn delete_tokens(&self, user_id: &Uuid, token_id: &Uuid) -> Result<(), Error> {
    let result = self.token_service.delete_access_tokens(user_id.clone(), token_id.clone()).await;
    if result.is_err() {
      return Err(result.unwrap_err());
    }

    let result = self.token_service.delete_refresh_tokens(user_id.clone(), token_id.clone()).await;
    if result.is_err() {
      return Err(result.unwrap_err());
    }

    Ok(())
  }
}

#[async_trait]
impl AuthService for AuthServiceImpl {
  async fn login(&self, user: User) -> Result<TokensResponseDto, Error> {
    let db_user = self.user_service.get_by_email(user.email.clone()).await;

    match db_user {
      Ok(Some(u)) => {
        let result = self.check_user(&user, &u);
        if result.is_err() {
          return Err(result.unwrap_err());
        }

        let user_id = u.id;
        let token_id = Uuid::new_v4();

        let result = self.generate_tokens(&user_id, &token_id);
        if result.is_err() {
          return Err(result.unwrap_err());
        }

        let tokens = result.unwrap();

        match self.save_tokens(&user_id, &token_id, &tokens).await {
          Ok(_) => Ok(tokens),
          Err(_) => Err(Error::InternalServerError("Error while saving tokens.".to_string()))
        }
      }
      Err(_) => Err(Error::InternalServerError("User not found.".to_string())),
      _ => Err(Error::InternalServerError("An error occurred.".to_string()))
    }
  }

  async fn logout(&self, auth_header: String) -> Result<(), Error> {
    let result = get_bearer_token(auth_header);
    if result.is_err() {
      return Err(result.unwrap_err());
    }

    let token_string = result.unwrap();

    let result = parse_jwt_token(token_string, self.settings.access_secret.clone());
    if result.is_err() {
      return Err(result.unwrap_err());
    }

    let claims = result.unwrap();

    if claims.is_refresh {
      return Err(Error::InternalServerError("Used refresh token for logout.".to_string()));
    }

    match self.delete_tokens(&claims.sub, &claims.jti).await {
      Ok(_) => Ok(()),
      Err(error) => Err(error)
    }
  }

  async fn refresh(&self, token_string: String) -> Result<TokensResponseDto, Error> {
    let result = parse_jwt_token(token_string, self.settings.refresh_secret.clone());
    if result.is_err() {
      return Err(result.unwrap_err());
    }

    let claims = result.unwrap();

    if !claims.is_refresh {
      return Err(Error::InternalServerError("Must be refresh token.".to_string()));
    }

    let user_id = &claims.sub;
    let token_id = &claims.jti;

    let result = self.delete_tokens(&user_id, &token_id).await;
    if result.is_err() {
      return Err(result.unwrap_err());
    }

    let result = self.generate_tokens(&user_id, &token_id);
    if result.is_err() {
      return Err(result.unwrap_err());
    }

    let tokens = result.unwrap();
    match self.save_tokens(&user_id, &token_id, &tokens).await {
      Ok(_) => Ok(tokens),
      Err(_) => Err(Error::InternalServerError("Error while saving tokens.".to_string()))
    }
  }
}