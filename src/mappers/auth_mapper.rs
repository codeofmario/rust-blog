use crate::dtos::request::login_request_dto::LoginRequestDto;
use crate::models::user::User;

pub fn from_login_dto_to_user(dto: LoginRequestDto) -> User {
    User {
        id: Default::default(),
        email: dto.email.clone(),
        username: Default::default(),
        password: dto.password.clone(),
        created_at: Default::default(),
        updated_at: Default::default(),
    }
}