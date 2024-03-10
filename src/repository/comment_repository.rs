use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::types::Uuid;

use crate::enums::error::Error;
use crate::models::comment::Comment;

#[async_trait]
pub trait CommentRepository: Send + Sync {
  async fn get_all(&self) -> Result<Vec<Comment>, Error>;
  async fn get_all_for_post(&self, post_id: Uuid) -> Result<Vec<Comment>, Error>;
  async fn get_one(&self, id: Uuid) -> Result<Comment, Error>;
  async fn create(&self, comment: Comment) -> Result<Comment, Error>;
  async fn update(&self, comment: Comment) -> Result<Comment, Error>;
}


pub struct CommentRepositoryImpl {
  pool: Arc<PgPool>,
}

impl CommentRepositoryImpl {
  pub fn new(pool: Arc<PgPool>) -> Self {
    CommentRepositoryImpl { pool }
  }
}

#[async_trait]
impl CommentRepository for CommentRepositoryImpl {
  async fn get_all(&self) -> Result<Vec<Comment>, Error> {
    let conn = self.pool.deref();
    sqlx::query_as!(
            Comment,
            r#"
            SELECT * FROM comments
            "#
        )
      .fetch_all(conn)
      .await
      .map_err(|_| Error::InternalServerError("Something went wrong.".to_string()))
  }

  async fn get_all_for_post(&self, post_id: Uuid) -> Result<Vec<Comment>, Error> {
    let conn = self.pool.deref();
    sqlx::query_as!(
            Comment,
            r#"
            SELECT * FROM comments
            WHERE post_id = $1
            "#,
            post_id
        )
      .fetch_all(conn)
      .await
      .map_err(|_| Error::InternalServerError("Something went wrong.".to_string()))
  }

  async fn get_one(&self, id: Uuid) -> Result<Comment, Error> {
    let conn = self.pool.deref();
    sqlx::query_as!(
            Comment,
            r#"
            SELECT * FROM comments
            WHERE id = $1
            "#,
            id
        )
      .fetch_one(conn)
      .await
      .map_err(|_| Error::NotFound("Not found.".to_string()))
  }

  async fn create(&self, comment: Comment) -> Result<Comment, Error> {
    let conn = self.pool.deref();
    sqlx::query_as!(
            Comment,
            r#"
            INSERT INTO comments (body, user_id, post_id)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            comment.body,
            comment.user_id,
            comment.post_id
        )
      .fetch_one(conn)
      .await
      .map_err(|_| Error::InternalServerError("Something went wrong.".to_string()))
  }

  async fn update(&self, comment: Comment) -> Result<Comment, Error> {
    let conn = self.pool.deref();
    sqlx::query_as!(
            Comment,
            r#"
            UPDATE comments
            SET body = $1, user_id = $2, post_id = $3
            WHERE id = $4
            RETURNING *
            "#,
            comment.body,
            comment.user_id,
            comment.post_id,
            comment.id
        )
      .fetch_one(conn)
      .await
      .map_err(|_| Error::InternalServerError("Something went wrong.".to_string()))
  }
}