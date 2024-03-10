use std::sync::Arc;

use axum::{Extension, Json};
use axum::http::{HeaderMap, StatusCode};
use axum::http::header::AUTHORIZATION;
use axum::response::IntoResponse;

use crate::dtos::request::login_request_dto::LoginRequestDto;
use crate::dtos::request::token_refresh_request_dto::TokenRefreshRequestDto;
use crate::dtos::response::error_response_dto::ErrorResponseDto;
use crate::mappers::auth_mapper::from_login_dto_to_user;
use crate::services::auth_service::AuthService;
use crate::traits::from_result_to_response::FromResultToResponse;
use crate::traits::from_result_to_response_with_no_content::FromResultToResponseWithNoContent;
use crate::utils::request_util::map_body_to_model;

pub struct AuthHandler;

impl AuthHandler {
  pub async fn login(
    Extension(service): Extension<Arc<dyn AuthService>>,
    Json(body): Json<LoginRequestDto>
  ) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponseDto>)> {
    let user = map_body_to_model(body, from_login_dto_to_user).await;

    service.login(user)
      .await
      .to_response(|tokens| tokens)
  }

  pub async fn logout(
    Extension(service): Extension<Arc<dyn AuthService>>,
    headers: HeaderMap
  ) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponseDto>)> {
    let authorization_header = headers.get(AUTHORIZATION).unwrap().to_str().unwrap().to_string();

    service.logout(authorization_header)
      .await
      .to_response_with_no_content()
  }

  pub async fn refresh_token(
    Extension(service): Extension<Arc<dyn AuthService>>,
    Json(body): Json<TokenRefreshRequestDto>
  ) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponseDto>)> {
    service.refresh(body.refresh_token)
      .await
      .to_response(|tokens| tokens)
  }
}
