use jsonwebtoken::{Algorithm, decode, DecodingKey, Validation};

use crate::enums::error::Error;
use crate::models::token_claims::TokenClaims;

pub fn get_bearer_token(auth_header: String) -> Result<String, Error> {
  let parts: Vec<&str> = auth_header.split_whitespace().collect();
  if parts.len() == 2 && parts[0] == "Bearer" {
    Ok(parts[1].to_string())
  } else {
    Err(Error::InternalServerError("Bearer token was not found inside Authentication header.".to_string()))
  }
}

pub fn parse_jwt_token(token_string: String, secret: String) -> Result<TokenClaims, Error> {
  let validation = Validation::new(Algorithm::HS256);

  match decode::<TokenClaims>(&token_string, &DecodingKey::from_secret(secret.as_bytes()), &validation) {
    Ok(token_data) => Ok(token_data.claims),
    Err(_) => Err(Error::InternalServerError("Error while decoding jwt token.".to_string())),
  }
}