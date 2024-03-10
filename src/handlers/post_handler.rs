use std::sync::Arc;

use axum::{Extension, Json};
use axum::extract::multipart::Multipart;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::dtos::request::post_request_dto::PostRequestDto;
use crate::dtos::response::error_response_dto::ErrorResponseDto;
use crate::mappers::post_mapper::{from_dto_to_post, from_post_to_dto};
use crate::models::token_claims::TokenClaims;
use crate::services::post_service::PostService;
use crate::traits::from_result_to_response::FromResultToResponse;
use crate::traits::from_result_to_response_with_no_content::FromResultToResponseWithNoContent;
use crate::utils::file_util::get_file_from_multipart;
use crate::utils::mapper_util::from_model_to_dto_list;

pub struct PostHandler;

impl PostHandler {
  pub async fn get_all(
    Extension(service): Extension<Arc<dyn PostService>>
  ) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponseDto>)> {
    service.get_all()
      .await
      .to_response(|posts| from_model_to_dto_list(posts, from_post_to_dto))
  }

  pub async fn get_one(
    Path(id): Path<Uuid>,
    Extension(service): Extension<Arc<dyn PostService>>,
  ) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponseDto>)> {
    service.get_one(id)
      .await
      .to_response(|post| from_post_to_dto(&post))
  }

  pub async fn create(
    Extension(service): Extension<Arc<dyn PostService>>,
    Extension(claims): Extension<Arc<TokenClaims>>,
    Json(body): Json<PostRequestDto>,
  ) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponseDto>)> {
    let mut post = from_dto_to_post(&body);
    post.user_id = claims.sub.clone();

    service.create(post)
      .await
      .to_response(|post| from_post_to_dto(&post))
  }

  pub async fn update(
    Path(id): Path<Uuid>,
    Extension(service): Extension<Arc<dyn PostService>>,
    Json(body): Json<PostRequestDto>,
  ) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponseDto>)> {
    let mut post = from_dto_to_post(&body);
    post.id = id;

    service.update(post)
      .await
      .to_response(|post| from_post_to_dto(&post))
  }

  pub async fn add_image(
    Path(id): Path<Uuid>,
    Extension(service): Extension<Arc<dyn PostService>>,
    multipart: Multipart,
  ) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponseDto>)> {
    let result = get_file_from_multipart("file".to_string(), multipart).await;
    if result.is_err() {
      let error = result.unwrap_err();
      return Err((error.code(), Json(ErrorResponseDto { error: error.message() })));
    }

    let (content, content_type) = result.unwrap();

    service.add_image(id, content, content_type)
      .await
      .to_response(|post| from_post_to_dto(&post))
  }

  pub async fn delete(
    Path(id): Path<Uuid>,
    Extension(service): Extension<Arc<dyn PostService>>,
  ) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponseDto>)> {
    service.delete(id)
      .await
      .to_response_with_no_content()
  }
}