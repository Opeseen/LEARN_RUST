use crate::organizations::{CreateOrganizationDto, Organization};
use async_trait::async_trait;
use sqlx::PgConnection;
use uuid::Uuid;

#[async_trait]
pub trait OrganizationRepository {
    async fn create(
        conn: &mut PgConnection,
        dto: &CreateOrganizationDto,
    ) -> Result<Uuid, sqlx::Error>;
    async fn find_all(conn: &mut PgConnection) -> Result<Vec<Organization>, sqlx::Error>;
    async fn find_by_id(
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<Organization>, sqlx::Error>;
}

pub struct OrganizationStorage {}

#[async_trait]
impl OrganizationRepository for OrganizationStorage {
    async fn create(
        conn: &mut PgConnection,
        dto: &CreateOrganizationDto,
    ) -> Result<Uuid, sqlx::Error> {
        let rec = sqlx::query_scalar::<_, Uuid>(
            "INSERT INTO organizations (name, official_email, address)
            VALUES ($1, $2, $3)
            RETURNING id",
        )
        .bind(&dto.name)
        .bind(&dto.official_email)
        .bind(&dto.address)
        .fetch_one(conn)
        .await?;
        Ok(rec)
    }

    async fn find_all(conn: &mut PgConnection) -> Result<Vec<Organization>, sqlx::Error> {
        let rec = sqlx::query_as::<_, Organization>(
            "SELECT * FROM organizations ORDER BY created_at DESC",
        )
        .fetch_all(conn)
        .await?;
        Ok(rec)
    }

    async fn find_by_id(
        conn: &mut PgConnection,
        id: Uuid,
    ) -> Result<Option<Organization>, sqlx::Error> {
        let rec = sqlx::query_as::<_, Organization>("SELECT * FROM organizations WHERE id = $1")
            .bind(id)
            .fetch_optional(conn)
            .await?;
        Ok(rec)
    }
}
