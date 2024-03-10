use std::sync::Arc;

use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::{HeaderMap, StatusCode};
use axum::response::IntoResponse;
use reqwest;
use s3::utils::GetAndConvertHeaders;

use crate::config::settings::Settings;
use crate::dtos::response::error_response_dto::ErrorResponseDto;

#[derive(Clone)]
pub struct ProxyHandler;

impl ProxyHandler {
  pub async fn serve_public_bucket(
    Path(id): Path<String>,
    Extension(settings): Extension<Arc<Settings>>,
  ) -> Result<impl IntoResponse, (StatusCode, Json<ErrorResponseDto>)> {
    let base_url = settings.minio_public_addr.as_str();
    let bucket = settings.minio_bucket_name.as_str();

    let response = reqwest::get(format!("{base_url}/{bucket}/{id}")).await.unwrap();

    let mut headers = HeaderMap::new();
    headers.insert("content-type", response.headers().get_string("content-type").unwrap().parse().unwrap());

    return Ok((headers, response.bytes().await.unwrap()));
  }
}