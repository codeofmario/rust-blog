use bcrypt::{DEFAULT_COST, hash, verify};

pub fn hash_password(password: String) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn check_password_hash(password: String, hash: String) -> bool {
    verify(password, hash.as_str()).unwrap()
}