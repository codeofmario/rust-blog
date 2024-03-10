use std::sync::Arc;
use std::vec::Vec;

use async_trait::async_trait;
use axum::body::Bytes;
use uuid::Uuid;

use crate::enums::error::Error;
use crate::models::post::Post;
use crate::repository::post_repository::PostRepository;
use crate::services::store_service::StoreService;

#[async_trait]
pub trait PostService: Send + Sync {
  async fn get_all(&self) -> Result<Vec<Post>, Error>;
  async fn get_one(&self, id: Uuid) -> Result<Post, Error>;
  async fn create(&self, post: Post) -> Result<Post, Error>;
  async fn update(&self, post: Post) -> Result<Post, Error>;
  async fn add_image(&self, id: Uuid, content: Bytes, content_type: String) -> Result<Post, Error>;
  async fn delete(&self, id: Uuid) -> Result<(), Error>;
}

#[derive(Clone)]
pub struct PostServiceImpl {
  repo: Arc<dyn PostRepository>,
  store_service: Arc<dyn StoreService>,
}

impl PostServiceImpl {
  pub fn new(repo: Arc<dyn PostRepository>, store_service: Arc<dyn StoreService>) -> Self {
    Self { repo, store_service }
  }
}

#[async_trait]
impl PostService for PostServiceImpl {
  async fn get_all(&self) -> Result<Vec<Post>, Error> {
    self.repo.get_all().await
  }

  async fn get_one(&self, id: Uuid) -> Result<Post, Error> {
    self.repo.get_one(id).await
  }

  async fn create(&self, post: Post) -> Result<Post, Error> {
    self.repo.create(post).await
  }

  async fn update(&self, post: Post) -> Result<Post, Error> {
    let result = self.get_one(post.id).await;
    if result.is_err() {
      return Err(result.unwrap_err());
    }

    let mut old_post = result.unwrap();
    old_post.title = post.title;
    old_post.body = post.body;

    self.repo.update(old_post).await
  }

  async fn add_image(&self, id: Uuid, content: Bytes, content_type: String) -> Result<Post, Error> {
    let result = self.store_service.save(content, content_type).await;
    if result.is_err() {
      return Err(result.unwrap_err());
    }

    let image_id = result.unwrap();

    let result = self.get_one(id).await;
    if result.is_err() {
      return Err(result.unwrap_err());
    }

    let mut post = result.unwrap();

    if post.image_id != Uuid::nil() {
      let result = self.store_service.delete(post.image_id).await;
      if result.is_err() {
        return Err(result.unwrap_err());
      }
    }

    post.image_id = image_id;

    self.repo.update(post).await
  }

  async fn delete(&self, id: Uuid) -> Result<(), Error> {
    let result = self.get_one(id).await;
    if result.is_err() {
      return Err(result.unwrap_err());
    }

    let post = result.unwrap();

    if post.image_id != Uuid::nil() {
      let result = self.store_service.delete(post.image_id).await;
      if result.is_err() {
        return Err(result.unwrap_err());
      }
    }

    self.repo.delete(id).await
  }
}