use argon2::{
    Argon2, PasswordHash, PasswordVerifier,
    password_hash::{PasswordHasher, SaltString, rand_core::OsRng},
};
use rand::{RngCore, thread_rng};

use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};

use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};

use serde::{Deserialize, Serialize};
use std::env;
use uuid::Uuid;

use crate::utility::{ApiError, ApiResponse, constants};

#[derive(Serialize, Deserialize, Debug)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub client_id: String,
    pub permissions: Vec<String>,
}

pub struct AuthGuard(pub Claims);
pub struct OrgIdHeader(pub Uuid);

pub fn hash_password(password: &str) -> Result<String, String> {
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

pub fn create_jwt(user_id: &Uuid, client_id: &str, permissions: Vec<String>) -> String {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(1))
        .expect("Valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        iat: Utc::now().timestamp() as usize,
        exp: expiration,
        client_id: client_id.to_owned(),
        permissions,
    };

    let secret = env::var("JWT_SECRET").expect("JWT Secret key not set");

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

fn verify_jwt(token: &str) -> Result<Claims, StatusCode> {
    let secret = env::var("JWT_SECRET").expect("JWT Secret key not set");
    let validation = Validation::new(Algorithm::HS256);

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &validation,
    );

    match token_data {
        Ok(data) => Ok(data.claims),
        Err(e) => {
            tracing::error!("Failed to verify jwt with error: {:?}", e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

#[async_trait]
// authentication middleware
impl<S> FromRequestParts<S> for AuthGuard
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Get the authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Extract the token from the header
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Verify the claims
        let claims = verify_jwt(token)?;

        Ok(AuthGuard(claims))
    }
}

// Organization Id middleware(Verifies org id is present in header)
#[async_trait]
impl<S> FromRequestParts<S> for OrgIdHeader
where
    S: Send + Sync,
{
    type Rejection = Response; // This already implement IntoResponse

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let org_id = parts
            .headers
            .get("X-Organization-Id")
            .and_then(|h_value| h_value.to_str().ok())
            .and_then(|input| Uuid::parse_str(input).ok())
            .ok_or_else(|| {
                (StatusCode::BAD_REQUEST, constants::MISSING_ORG_HEADER).into_response()
            })?;

        Ok(OrgIdHeader(org_id))
    }
}

pub fn require_permission(claims: &Claims, permission: &str) -> Result<(), ApiError> {
    if !claims.permissions.contains(&permission.to_string()) {
        return Err(ApiResponse::error(
            StatusCode::FORBIDDEN,
            "You do not have permission to access this resource",
        ));
    }
    Ok(())
}
