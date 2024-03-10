use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRequestDto {
    pub title: String,
    pub body: String,
}