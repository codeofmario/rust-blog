use std::sync::Arc;

use rust_blog::config::demo::init_demo;
use rust_blog::container::Container;
use rust_blog::create_app::create_app;

#[tokio::main]
async fn main() {
  let container = Arc::new(Container::new().await);

  let user_service = container.user_service.clone();
  init_demo(user_service).await;

  let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
  axum::serve(listener, create_app(container.clone())).await.unwrap()
}