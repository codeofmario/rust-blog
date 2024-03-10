use std::sync::Arc;

use async_trait::async_trait;
use uuid::Uuid;

use crate::enums::error::Error;
use crate::models::user::{CreateUser, User};
use crate::repository::user_repository::UserRepository;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_all(&self) -> Result<Vec<User>, Error>;
    async fn get_one(&self, id: Uuid) -> Result<Option<User>, Error>;
    async fn get_by_email(&self, email: String) -> Result<Option<User>, Error>;
    async fn create(&self, user: CreateUser) -> Result<User, Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Error>;
}


pub struct UserServiceImpl {
    repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        UserServiceImpl { repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_all(&self) -> Result<Vec<User>, Error> {
        self.repository.all().await
    }

    async fn get_one(&self, id: Uuid) -> Result<Option<User>, Error> {
        self.repository.get_by_id(id).await
    }

    async fn get_by_email(&self, email: String) -> Result<Option<User>, Error> {
        self.repository.get_by_email(email).await
    }

    async fn create(&self, user: CreateUser) -> Result<User, Error> {
        self.repository.create(user.clone()).await
    }

    async fn delete(&self, id: Uuid) -> Result<(), Error> {
        self.repository.delete(id).await
    }
}
