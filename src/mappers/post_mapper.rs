use uuid::Uuid;

use crate::dtos::request::post_request_dto::PostRequestDto;
use crate::dtos::response::post_response_dto::PostResponseDto;
use crate::models::post::Post;

pub fn from_dto_to_post(dto: &PostRequestDto) -> Post {
    Post {
        id: Default::default(),
        title: dto.title.clone(),
        body: dto.body.clone(),
        image_id: Default::default(),
        user_id: Default::default(),
        created_at: Default::default(),
        updated_at: Default::default(),
    }
}

pub fn from_post_to_dto(model: &Post) -> PostResponseDto {
    let image_url = if model.image_id != Uuid::nil() {
        format!("/assets/images/{}", model.image_id.to_string())
    } else {
        String::new()
    };

    PostResponseDto {
        id: model.id.to_string(),
        title: model.title.clone(),
        body: model.body.clone(),
        image_url,
        user_id: model.user_id.to_string(),
        created_at: model.created_at,
        updated_at: model.updated_at,
    }
}
