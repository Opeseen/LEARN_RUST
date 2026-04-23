use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use rand::{RngCore, thread_rng};

pub async fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    // hash the password
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| {
            tracing::error!("Failed to hash password: {}", e);
            format!("Something went wrong")
        })?
        .to_string();

    Ok(password_hash)
}

pub fn verify_user_password(password: &str, hash: &str) -> bool {
    // parse the hash string back to the PasswordHash struct
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(hash) => hash,
        Err(e) => {
            tracing::error!("Failed to parse password hash: {}", e);
            return false;
        }
    };
    // verify the password against the hash
    let is_valid = Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok();

    is_valid
}

pub fn generate_auth_code() -> String {
    let mut buf = [0u8; 32]; // 32 bytes = 256 bits
    thread_rng().fill_bytes(&mut buf);
    hex::encode(buf)
}
