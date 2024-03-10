use std::sync::Arc;

use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;

use crate::dtos::request::comment_request_dto::CommentRequestDto;
use crate::dtos::response::error_response_dto::ErrorResponseDto;
use crate::mappers::comment_mapper::{from_comment_to_dto, from_dto_to_comment};
use crate::models::token_claims::TokenClaims;
use crate::services::comment_service::CommentService;
use crate::traits::from_result_to_response::FromResultToResponse;
use crate::utils::mapper_util::from_model_to_dto_list;

pub struct CommentHandler;

impl CommentHandler {
  pub async fn get_all(
    Path(post_id): Path<Uuid>,
    Extension(service): Extension<Arc<dyn CommentService>>
  ) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponseDto>)> {
    service.get_all_for_post(post_id)
      .await
      .to_response(|comments| from_model_to_dto_list(comments, from_comment_to_dto))
  }

  pub async fn create(
    Extension(service): Extension<Arc<dyn CommentService>>,
    Extension(claims): Extension<Arc<TokenClaims>>,
    Json(body): Json<CommentRequestDto>,
  ) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponseDto>)> {
    let mut comment = from_dto_to_comment(&body);
    comment.user_id = claims.sub.clone();

    service.create(comment)
      .await
      .to_response(|comment| from_comment_to_dto(&comment))
  }

  pub async fn update(
    Path((_, id)): Path<(Uuid, Uuid)>,
    Extension(service): Extension<Arc<dyn CommentService>>,
    Json(body): Json<CommentRequestDto>,
  ) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponseDto>)> {
    let mut comment = from_dto_to_comment(&body);
    comment.id = id;

    service.update(comment)
      .await
      .to_response(|comment| from_comment_to_dto(&comment))
  }
}