use std::sync::Arc;

use axum::extract::Request;
use axum::http::{HeaderMap, StatusCode};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};

use crate::config::settings::init_settings;
use crate::utils::token_util::{get_bearer_token, parse_jwt_token};

pub async fn jwt_auth_middleware(
  headers: HeaderMap,
  mut request: Request,
  next: Next,
) -> Response {

  let header = headers.get(AUTHORIZATION).unwrap().to_str().unwrap().to_string();

  let result = get_bearer_token(header);
  if result.is_err() {
    return StatusCode::UNAUTHORIZED.into_response();
  };

  let token = result.unwrap();

  let secret = init_settings().access_secret;
  let result = parse_jwt_token(token, secret);
  if result.is_err() {
    return StatusCode::UNAUTHORIZED.into_response();
  }

  let claims = result.unwrap();

  request.extensions_mut().insert(Arc::new(claims));

  return next.run(request).await;
}