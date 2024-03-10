use std::sync::Arc;

use async_trait::async_trait;
use chrono::Duration;
use redis::aio::Connection;
use redis::AsyncCommands;
use tokio::sync::Mutex;

use crate::enums::error::Error;

#[async_trait]
pub trait TokenRepository: Send + Sync {
  async fn get_token(&self, key: String) -> Result<String, Error>;
  async fn save_token(&self, key: String, token: String, exp: Duration) -> Result<bool, Error>;
  async fn delete_token(&self, key: String) -> Result<bool, Error>;
}

pub struct TokenRepositoryImpl {
  redis: Arc<Mutex<Connection>>,
}

impl TokenRepositoryImpl {
  pub fn new(redis: Arc<Mutex<Connection>>) -> Self {
    TokenRepositoryImpl { redis }
  }
}

#[async_trait]
impl TokenRepository for TokenRepositoryImpl {
  async fn get_token(&self, key: String) -> Result<String, Error> {
    return self.redis
      .lock()
      .await
      .get(key)
      .await
      .map_err(|_| Error::InternalServerError("Something went wrong.".to_string()));
  }

  async fn save_token(&self, key: String, token: String, exp: Duration) -> Result<bool, Error> {
    self.redis
      .lock()
      .await
      .set_ex(key, token, exp.num_seconds() as u64)
      .await
      .map_err(|_| Error::InternalServerError("Something went wrong.".to_string()))
  }

  async fn delete_token(&self, key: String) -> Result<bool, Error> {
    self.redis
      .lock()
      .await
      .del(key)
      .await
      .map_err(|_| Error::InternalServerError("Something went wrong.".to_string()))
  }
}