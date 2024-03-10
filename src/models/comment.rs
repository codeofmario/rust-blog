use sqlx::types::Uuid;

#[derive(Debug, sqlx::FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub body: String,
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}