use axum::body::Bytes;
use axum::extract::Multipart;

use crate::enums::error::Error;

pub async fn get_file_from_multipart(name: String, mut multipart: Multipart) -> Result<(Bytes, String), Error> {

  let result = multipart.next_field().await;
  if result.is_err() {
    return Err(Error::InternalServerError("Something went wrong.".to_string()));
  }

  match result.unwrap() {
    Some(field) => {
      let field_name = field.name().unwrap();
      if field_name != name {
        return Err(Error::BadRequest("File was not provided.".to_string()));
      }

      let content_type = field.content_type().unwrap().to_string();
      let content = field.bytes().await.unwrap();

      Ok((content, content_type))
    }
    None => Err(Error::InternalServerError("Something went wrong.".to_string()))
  }
}