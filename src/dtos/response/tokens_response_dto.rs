use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TokensResponseDto {
    pub access_token: String,
    pub refresh_token: String,
}
