use std::sync::Arc;
use std::vec::Vec;

use async_trait::async_trait;
use uuid::Uuid;

use crate::enums::error::Error;
use crate::models::comment::Comment;
use crate::repository::comment_repository::CommentRepository;

#[async_trait]
pub trait CommentService: Send + Sync {
  async fn get_all(&self) -> Result<Vec<Comment>, Error>;
  async fn get_all_for_post(&self, post_id: Uuid) -> Result<Vec<Comment>, Error>;
  async fn get_one(&self, id: Uuid) -> Result<Comment, Error>;
  async fn create(&self, comment: Comment) -> Result<Comment, Error>;
  async fn update(&self, comment: Comment) -> Result<Comment, Error>;
}

#[derive(Clone)]
pub struct CommentServiceImpl {
  repo: Arc<dyn CommentRepository>,
}

impl CommentServiceImpl {
  pub fn new(repo: Arc<dyn CommentRepository>) -> Self {
    CommentServiceImpl { repo }
  }
}

#[async_trait]
impl CommentService for CommentServiceImpl {
  async fn get_all(&self) -> Result<Vec<Comment>, Error> {
    self.repo.get_all().await
  }

  async fn get_all_for_post(&self, post_id: Uuid) -> Result<Vec<Comment>, Error> {
    self.repo.get_all_for_post(post_id).await
  }

  async fn get_one(&self, id: uuid::Uuid) -> Result<Comment, Error> {
    self.repo.get_one(id).await
  }

  async fn create(&self, comment: Comment) -> Result<Comment, Error> {
    self.repo.create(comment).await
  }

  async fn update(&self, comment: Comment) -> Result<Comment, Error> {
    let result = self.get_one(comment.id).await;
    if result.is_err() {
      return Err(result.unwrap_err());
    }

    let mut old_comment = result.unwrap();
    old_comment.body = comment.body;

    self.repo.update(old_comment).await
  }
}