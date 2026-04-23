use crate::organizations::{CreateOrganizationDto, Organization};
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug, thiserror::Error)]
pub enum RepoError {
    #[error("Organization with email '{0}' already exists")]
    AlreadyExists(String),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

#[async_trait]
pub trait OrganizationRepository {
    async fn create(conn: &PgPool, dto: &CreateOrganizationDto) -> Result<Uuid, RepoError>;
    async fn find_all(conn: &PgPool) -> Result<Vec<Organization>, sqlx::Error>;
    async fn find_by_id(conn: &PgPool, id: Uuid) -> Result<Option<Organization>, sqlx::Error>;
    async fn find_by_email(conn: &PgPool, email: &str)
    -> Result<Option<Organization>, sqlx::Error>;
}

pub struct OrganizationStorage {}

#[async_trait]
impl OrganizationRepository for OrganizationStorage {
    async fn create(conn: &PgPool, dto: &CreateOrganizationDto) -> Result<Uuid, RepoError> {
        // check if the email exists first
        let existing = Self::find_by_email(conn, &dto.official_email).await?;
        if existing.is_some() {
            return Err(RepoError::AlreadyExists(dto.official_email.clone()));
        }

        let rec = sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO organizations (name, official_email, address)
            VALUES ($1, $2, $3)
            RETURNING id",
        )
        .bind(&dto.name)
        .bind(&dto.official_email.to_lowercase())
        .bind(&dto.address)
        .fetch_one(conn)
        .await?;
        Ok(rec)
    }

    async fn find_all(conn: &PgPool) -> Result<Vec<Organization>, sqlx::Error> {
        let rec = sqlx::query_as::<_, Organization>(
            "SELECT * FROM organizations ORDER BY created_at DESC",
        )
        .fetch_all(conn)
        .await?;
        Ok(rec)
    }

    async fn find_by_id(conn: &PgPool, id: Uuid) -> Result<Option<Organization>, sqlx::Error> {
        let rec = sqlx::query_as::<_, Organization>("SELECT * FROM organizations WHERE id = $1")
            .bind(id)
            .fetch_optional(conn)
            .await?;
        Ok(rec)
    }

    async fn find_by_email(
        conn: &PgPool,
        email: &str,
    ) -> Result<Option<Organization>, sqlx::Error> {
        let rec = sqlx::query_as::<_, Organization>(
            "SELECT * FROM organizations WHERE official_email = $1",
        )
        .bind(email.to_lowercase())
        .fetch_optional(conn)
        .await?;
        Ok(rec)
    }
}
