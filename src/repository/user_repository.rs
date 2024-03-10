use std::ops::Deref;
use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{PgPool, query, query_as};
use uuid::Uuid;

use crate::enums::error::Error;
use crate::models::user::{CreateUser, User};

#[async_trait]
pub trait UserRepository: Send + Sync {
  async fn all(&self) -> Result<Vec<User>, Error>;
  async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, Error>;
  async fn get_by_email(&self, email: String) -> Result<Option<User>, Error>;
  async fn create(&self, user: CreateUser) -> Result<User, Error>;
  async fn update(&self, id: Uuid, user: User) -> Result<User, Error>;
  async fn delete(&self, id: Uuid) -> Result<(), Error>;
}

#[derive(Debug, Clone)]
pub struct UserRepositoryImpl {
  pool: Arc<PgPool>,
}

impl UserRepositoryImpl {
  pub fn new(db_pool: Arc<PgPool>) -> Self {
    UserRepositoryImpl { pool: db_pool }
  }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
  async fn all(&self) -> Result<Vec<User>, Error> {
    let conn = self.pool.deref();
    query_as::<_, User>("SELECT * FROM users")
      .fetch_all(conn)
      .await
      .map_err(|_| Error::InternalServerError("Something went wrong.".to_string()))
  }

  async fn get_by_id(&self, id: Uuid) -> Result<Option<User>, Error> {
    let conn = self.pool.deref();
    query_as::<_, User>("SELECT * FROM users WHERE id = $1")
      .bind(&id)
      .fetch_optional(conn)
      .await
      .map_err(|_| Error::NotFound("Not found.".to_string()))
  }

  async fn get_by_email(&self, email: String) -> Result<Option<User>, Error> {
    let conn = self.pool.deref();
    query_as::<_, User>("SELECT * FROM users WHERE email = $1")
      .bind(&email)
      .fetch_optional(conn)
      .await
      .map_err(|_| Error::NotFound("Not found.".to_string()))
  }

  async fn create(&self, user: CreateUser) -> Result<User, Error> {
    let conn = self.pool.deref();
    query_as::<_, User>(
      "INSERT INTO users (email, username, password) VALUES ($1, $2, $3) RETURNING *",
    )
      .bind(&user.email)
      .bind(&user.username)
      .bind(&user.password)
      .fetch_one(conn)
      .await
      .map_err(|_| Error::InternalServerError("Something went wrong.".to_string()))
  }

  async fn update(&self, id: Uuid, user: User) -> Result<User, Error> {
    let conn = self.pool.deref();
    query_as::<_, User>(
      "UPDATE users SET email = $1, username = $2, password = $3 WHERE id = $4 RETURNING *",
    )
      .bind(&user.email)
      .bind(&user.username)
      .bind(&user.password)
      .bind(&id)
      .fetch_one(conn)
      .await
      .map_err(|_| Error::InternalServerError("Something went wrong.".to_string()))
  }

  async fn delete(&self, id: Uuid) -> Result<(), Error> {
    let conn = self.pool.deref();
    query("DELETE FROM users WHERE id = $1")
      .bind(&id)
      .execute(conn)
      .await
      .map(|_| ())
      .map_err(|_| Error::InternalServerError("Something went wrong.".to_string()))

  }
}
