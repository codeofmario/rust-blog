use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub iss: String,      // Issuer
    pub sub: Uuid,      // Subject (user ID)
    pub jti: Uuid,      // Token ID
    pub iat: i64,         // Issued At (Unix timestamp)
    pub exp: i64,         // Expiration (Unix timestamp)
    pub is_refresh: bool, // Custom claim indicating whether it's a refresh token
}