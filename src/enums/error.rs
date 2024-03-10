use axum::http::StatusCode;

#[derive(Debug)]
pub enum Error {
  BadRequest(String),
  Unauthorized(String),
  Forbidden(String),
  NotFound(String),
  InternalServerError(String),
}

impl Error {
  pub fn code(&self) -> StatusCode {
    match self  {
      Error::BadRequest(_) => StatusCode::BAD_REQUEST,
      Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
      Error::Forbidden(_) => StatusCode::FORBIDDEN,
      Error::NotFound(_) => StatusCode::NOT_FOUND,
      Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR
    }
  }

  pub fn message(&self) -> String {
    match self  {
      Error::BadRequest(v) => v.clone(),
      Error::Unauthorized(v) => v.clone(),
      Error::Forbidden(v) => v.clone(),
      Error::NotFound(v) => v.clone(),
      Error::InternalServerError(v) => v.clone()
    }
  }
}