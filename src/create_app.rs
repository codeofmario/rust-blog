use std::sync::Arc;

use axum::{Extension, Router};
use axum::middleware::from_fn;
use axum::routing::{delete, get, post, put};
use tower_http::services::{ServeDir, ServeFile};

use crate::config::settings::init_settings;
use crate::container::Container;
use crate::handlers::auth_handler::AuthHandler;
use crate::handlers::comment_handler::CommentHandler;
use crate::handlers::post_handler::PostHandler;
use crate::handlers::proxy_handler::ProxyHandler;
use crate::middlewares::jwt_auth_middleware::jwt_auth_middleware;

pub fn create_app(container: Arc<Container>) -> Router {
  let routes = Router::new()
    // Swagger
    .nest_service("/docs", ServeDir::new("./assets/swagger-ui/"))
    .route_service("/docs-src", ServeFile::new("./docs/openapi.yml"))

    // Auth
    .route("/auth/login", post(AuthHandler::login))
    .route("/auth/token/refresh", post(AuthHandler::refresh_token));

  let routes_with_auth = Router::new()
    // Auth
    .route("/auth/logout", post(AuthHandler::logout))

    // Posts
    .route("/posts", get(PostHandler::get_all))
    .route("/posts/:id", get(PostHandler::get_one))
    .route("/posts", post(PostHandler::create))
    .route("/posts/:id", put(PostHandler::update))
    .route("/posts/:id/image", put(PostHandler::add_image))
    .route("/posts/:id", delete(PostHandler::delete))

    // Comments
    .route("/posts/:id/comments", get(CommentHandler::get_all))
    .route("/posts/:id/comments", post(CommentHandler::create))
    .route("/posts/:post_id/comments/:id", put(CommentHandler::update))

    .route_layer(from_fn(jwt_auth_middleware));

  let app = Router::new()
    .nest("/api", routes)
    .nest("/api", routes_with_auth)
    // File proxy
    .route("/assets/images/:id", get(ProxyHandler::serve_public_bucket));

  app.layer(Extension(Arc::new(init_settings())))
    .layer(Extension(container.token_service.clone()))
    .layer(Extension(container.user_service.clone()))
    .layer(Extension(container.auth_service.clone()))
    .layer(Extension(container.store_service.clone()))
    .layer(Extension(container.post_service.clone()))
    .layer(Extension(container.comment_service.clone()))
}