use std::sync::Arc;

use async_trait::async_trait;
use axum::body::Bytes;
use s3::Bucket;
use uuid::Uuid;

use crate::enums::error::Error;

#[async_trait]
pub trait StoreService: Send + Sync {
  async fn save(&self, content: Bytes, content_type: String) -> Result<Uuid, Error>;
  async fn delete(&self, id: Uuid) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct StoreServiceImpl {
  store: Arc<Bucket>,
}

impl StoreServiceImpl {
  pub fn new(store: Arc<Bucket>) -> Self {
    StoreServiceImpl {
      store,
    }
  }
}

#[async_trait]
impl StoreService for StoreServiceImpl {
  async fn save(&self, content: Bytes, content_type: String) -> Result<Uuid, Error> {
    let id = Uuid::new_v4();

    let result = self.store
      .put_object_with_content_type(id.to_string(),
                                    &content.to_vec(),
                                    content_type.as_str()).await;
    if result.is_err() {
      return Err(Error::InternalServerError("Error saving file.".to_string()));
    }

    Ok(id)
  }

  async fn delete(&self, id: Uuid) -> Result<(), Error> {
    let result = self.store.delete_object(id.to_string()).await;

    if result.is_err() {
      return Err(Error::InternalServerError("Error deleting file.".to_string()));
    }

    Ok(())
  }
}