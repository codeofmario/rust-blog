use serde::Serialize;
use chrono::prelude::*;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentResponseDto {
    pub id: String,
    pub body: String,
    pub user_id: String,
    pub post_id: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}