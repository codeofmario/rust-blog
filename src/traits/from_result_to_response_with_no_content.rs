use axum::http::StatusCode;
use axum::Json;

use crate::dtos::response::error_response_dto::ErrorResponseDto;
use crate::enums::error::Error;

pub trait FromResultToResponseWithNoContent {
  fn to_response_with_no_content(self) -> Result<(StatusCode, ()), (StatusCode, Json<ErrorResponseDto>)>;
}

impl FromResultToResponseWithNoContent for Result<(), Error> {
  fn to_response_with_no_content(self) -> Result<(StatusCode, ()), (StatusCode, Json<ErrorResponseDto>)>
  {
    match self {
      Ok(()) => Ok((StatusCode::NO_CONTENT, ())),
      Err(error) => Err((error.code(), Json(ErrorResponseDto { error: error.message() }))),
    }
  }
}