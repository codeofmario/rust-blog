use crate::dtos::request::comment_request_dto::CommentRequestDto;
use crate::dtos::response::comment_response_dto::CommentResponseDto;
use crate::models::comment::Comment;

pub fn from_dto_to_comment(dto: &CommentRequestDto) -> Comment {
    Comment {
        id: Default::default(),
        body: dto.body.clone(),
        user_id: Default::default(),
        post_id: dto.post_id,
        created_at: Default::default(),
        updated_at: Default::default(),
    }
}

pub fn from_comment_to_dto(model: &Comment) -> CommentResponseDto {
    CommentResponseDto {
        id: model.id.to_string(),
        body: model.body.clone(),
        user_id: model.user_id.to_string(),
        post_id: model.post_id.to_string(),
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}
