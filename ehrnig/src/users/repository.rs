use crate::users::{CreateUserDto, User};
use sqlx::PgConnection;
use uuid::Uuid;

pub async fn create(
    conn: &mut PgConnection,
    dto: &CreateUserDto,
    org_id: Uuid,
) -> Result<Uuid, sqlx::Error> {
    let rec = sqlx::query_scalar(
        "INSERT INTO users (organization_id, first_name, last_name, email, password_hash, role)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id",
    )
    .bind(org_id)
    .bind(&dto.first_name)
    .bind(&dto.last_name)
    .bind(&dto.email)
    .bind(&dto.password)
    .bind(&dto.role)
    .fetch_one(conn)
    .await?;

    Ok(rec)
}

pub async fn find_by_email(
    conn: &mut PgConnection,
    email: &str,
) -> Result<Option<User>, sqlx::Error> {
    let rec = sqlx::query_as("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(conn)
        .await?;

    Ok(rec)
}
