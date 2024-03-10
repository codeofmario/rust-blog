use std::sync::Arc;

use crate::models::user::CreateUser;
use crate::services::user_service::UserService;
use crate::utils::password_util::hash_password;

async fn create_user(service: Arc<dyn UserService>, username: String, email: String, password: String) {
  let exists = service.get_by_email(email.clone())
    .await
    .unwrap()
    .is_some();

  if exists { return; }

  let password = hash_password(password).unwrap();
  let user = CreateUser { email, username: username.clone(), password };
  service.create(user)
    .await
    .expect(format!("Failed to create {}.", username).as_str());
}

pub async fn init_demo(service: Arc<dyn UserService>) {
  // John
  create_user(service.clone(), "jonn".to_string(), "john@rustblog.com".to_string(), "password".to_string()).await;

  // Jane
  create_user(service.clone(), "jane".to_string(), "jane@rustblog.com".to_string(), "password".to_string()).await;
}