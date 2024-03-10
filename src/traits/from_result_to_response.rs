use axum::http::StatusCode;
use axum::Json;

use crate::dtos::response::error_response_dto::ErrorResponseDto;
use crate::enums::error::Error;

pub trait FromResultToResponse<T> {
  fn to_response<U, F>(self, op: F) -> Result<(StatusCode, Json<U>), (StatusCode, Json<ErrorResponseDto>)>
    where
      F: FnOnce(T) -> U;
}

impl<T> FromResultToResponse<T> for Result<T, Error> {
  fn to_response<U, F>(self, op: F) -> Result<(StatusCode, Json<U>), (StatusCode, Json<ErrorResponseDto>)>
    where
      F: FnOnce(T) -> U,
  {
    match self {
      Ok(value) => Ok((StatusCode::OK, Json(op(value)))),
      Err(error) => Err((error.code(), Json(ErrorResponseDto { error: error.message() }))),
    }
  }
}