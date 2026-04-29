use crate::users::{RegisterUserDto, User};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("User with email '{0}' already exists")]
    AlreadyExists(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

pub async fn create_user(
    conn: &PgPool,
    dto: &RegisterUserDto,
    org_id: Uuid,
) -> Result<Uuid, RepoError> {
    // check if the user exists
    let existing = find_by_email(conn, &dto.email, &org_id).await?;
    if existing.is_some() {
        return Err(RepoError::AlreadyExists(dto.email.clone()));
    }

    let rec = sqlx::query_scalar(
        "INSERT INTO users (organization_id, email, password_hash, first_name, last_name)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id",
    )
    .bind(org_id)
    .bind(dto.email.to_lowercase())
    .bind(&dto.password)
    .bind(&dto.first_name)
    .bind(&dto.last_name)
    .fetch_one(conn)
    .await?;

    Ok(rec)
}

pub async fn find_by_email(
    conn: &PgPool,
    email: &str,
    org_id: &Uuid,
) -> Result<Option<User>, sqlx::Error> {
    let rec = sqlx::query_as("SELECT * FROM users WHERE organization_id = $1 AND email = $2")
        .bind(org_id)
        .bind(email.to_lowercase())
        .fetch_optional(conn)
        .await?;

    Ok(rec)
}
