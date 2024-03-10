use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::types::Uuid;

use crate::enums::error::Error;
use crate::models::post::Post;

#[derive(Debug, sqlx::FromRow)]
pub struct NewPost {
  pub title: String,
  pub body: String,
  pub image_id: Uuid,
  pub user_id: Uuid,
}

#[async_trait]
pub trait PostRepository: Send + Sync {
  async fn get_all(&self) -> Result<Vec<Post>, Error>;
  async fn get_one(&self, id: Uuid) -> Result<Post, Error>;
  async fn create(&self, post: Post) -> Result<Post, Error>;
  async fn update(&self, post: Post) -> Result<Post, Error>;
  async fn delete(&self, id: Uuid) -> Result<(), Error>;
}

pub struct PostRepositoryImpl {
  pool: Arc<PgPool>,
}

impl PostRepositoryImpl {
  pub fn new(pool: Arc<PgPool>) -> Self {
    PostRepositoryImpl { pool }
  }
}

#[async_trait]
impl PostRepository for PostRepositoryImpl {
  async fn get_all(&self) -> Result<Vec<Post>, Error> {
    let conn = self.pool.deref();
    sqlx::query_as!(
            Post,
            r#"
            SELECT * FROM posts
            "#
        )
      .fetch_all(conn)
      .await
      .map_err(|_| Error::InternalServerError("Something went wrong.".to_string()))
  }

  async fn get_one(&self, id: Uuid) -> Result<Post, Error> {
    let conn = self.pool.deref();
    sqlx::query_as!(
            Post,
            r#"
            SELECT * FROM posts
            WHERE id = $1
            "#,
            id
        )
      .fetch_one(conn)
      .await
      .map_err(|_| Error::NotFound("Not found.".to_string()))
  }

  async fn create(&self, post: Post) -> Result<Post, Error> {
    let conn = self.pool.deref();
    sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (title, body, image_id, user_id)
            VALUES ($1, $2, $3, $4)
            RETURNING *
            "#,
            post.title,
            post.body,
            post.image_id,
            post.user_id
        )
      .fetch_one(conn)
      .await
      .map_err(|_| Error::InternalServerError("Something went wrong.".to_string()))
  }

  async fn update(&self, post: Post) -> Result<Post, Error> {
    let conn = self.pool.deref();
    sqlx::query_as!(
            Post,
            r#"
            UPDATE posts
            SET title = $1, body = $2, image_id = $3, user_id = $4
            WHERE id = $5
            RETURNING *
            "#,
            post.title,
            post.body,
            post.image_id,
            post.user_id,
            post.id
        )
      .fetch_one(conn)
      .await
      .map_err(|_| Error::InternalServerError("Something went wrong.".to_string()))
  }

  async fn delete(&self, id: Uuid) -> Result<(), Error> {
    let conn = self.pool.deref();
    sqlx::query!(
            r#"
            DELETE FROM posts
            WHERE id = $1
            "#,
            id
        ).execute(conn)
      .await
      .map(|_| ())
      .map_err(|_| Error::InternalServerError("Something went wrong.".to_string()))
  }
}
