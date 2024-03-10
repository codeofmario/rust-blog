use std::sync::Arc;

use redis::aio::Connection;
use s3::Bucket;
use sqlx::PgPool;
use tokio::sync::Mutex;

use crate::config::pool::init_pool;
use crate::config::redis::init_redis;
use crate::config::settings::init_settings;
use crate::config::store::init_store;
use crate::repository::comment_repository::{CommentRepository, CommentRepositoryImpl};
use crate::repository::post_repository::{PostRepository, PostRepositoryImpl};
use crate::repository::token_repository::{TokenRepository, TokenRepositoryImpl};
use crate::repository::user_repository::{UserRepository, UserRepositoryImpl};
use crate::services::auth_service::{AuthService, AuthServiceImpl};
use crate::services::comment_service::{CommentService, CommentServiceImpl};
use crate::services::post_service::{PostService, PostServiceImpl};
use crate::services::store_service::{StoreService, StoreServiceImpl};
use crate::services::token_service::{TokenService, TokenServiceImpl};
use crate::services::user_service::{UserService, UserServiceImpl};

pub struct Container {
  pub token_service: Arc<dyn TokenService>,
  pub user_service: Arc<dyn UserService>,
  pub auth_service: Arc<dyn AuthService>,
  pub store_service: Arc<dyn StoreService>,
  pub comment_service: Arc<dyn CommentService>,
  pub post_service: Arc<dyn PostService>,
}

impl Container {
  pub async fn new() -> Self {
    // Repositories
    let comment_repository: Arc<dyn CommentRepository> = Arc::new(
      CommentRepositoryImpl::new(Container::create_pool().await)
    );
    let post_repository: Arc<dyn PostRepository> = Arc::new(
      PostRepositoryImpl::new(Container::create_pool().await)
    );
    let token_repository: Arc<dyn TokenRepository> = Arc::new(
      TokenRepositoryImpl::new(Container::create_redis().await)
    );
    let user_repository: Arc<dyn UserRepository> = Arc::new(
      UserRepositoryImpl::new(Container::create_pool().await)
    );

    // Services
    let token_service: Arc<dyn TokenService> = Arc::new(
      TokenServiceImpl::new(Arc::new(init_settings()), token_repository)
    );
    let user_service: Arc<dyn UserService> = Arc::new(
      UserServiceImpl::new(user_repository)
    );
    let auth_service: Arc<dyn AuthService> = Arc::new(
      AuthServiceImpl::new(user_service.clone(), token_service.clone(), Arc::new(init_settings()))
    );
    let comment_service = Arc::new(CommentServiceImpl::new(comment_repository));
    let store_service: Arc<dyn StoreService> = Arc::new(
      StoreServiceImpl::new(Container::create_store())
    );
    let post_service = Arc::new(PostServiceImpl::new(post_repository, store_service.clone()));

    Container {
      token_service,
      user_service,
      auth_service,
      store_service,
      comment_service,
      post_service,
    }
  }

  async fn create_pool() -> Arc<PgPool> { Arc::new(init_pool(init_settings()).await) }
  fn create_store() -> Arc<Bucket> { Arc::new(init_store(init_settings())) }
  async fn create_redis() -> Arc<Mutex<Connection>> { Arc::new(Mutex::new(init_redis(init_settings()).await)) }
}
