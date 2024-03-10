use serde::Serialize;
use chrono::prelude::*;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PostResponseDto {
    pub id: String,
    pub title: String,
    pub body: String,
    pub image_url: String,
    pub user_id: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
