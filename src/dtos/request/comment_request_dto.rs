use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommentRequestDto {
    pub body: String,
    pub post_id: Uuid,
}