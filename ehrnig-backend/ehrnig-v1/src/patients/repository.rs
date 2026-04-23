use crate::patients::{CreatePatientDto, Patient};
use sqlx::PgConnection;
use uuid::Uuid;

fn generate_mrn() -> String {
    format!(
        "MRN-{}",
        uuid::Uuid::new_v4().to_string()[..8].to_uppercase()
    )
}

pub async fn save(
    conn: &mut PgConnection,
    org_id: Uuid,
    dto: &CreatePatientDto,
) -> Result<Uuid, sqlx::Error> {
    let rec = sqlx::query_scalar::<_,Uuid>(
        "INSERT INTO patients (organization_id, first_name, last_name,
         phone_number, date_of_birth, gender, medical_record_number) VALUES ($1, $2, $3, $4, $5, $6, $7) 
         RETURNING id",
    )
    .bind(org_id)
    .bind(&dto.first_name)
    .bind(&dto.last_name)
    .bind(&dto.phone_number)
    .bind(&dto.date_of_birth)
    .bind(&dto.gender)
    .bind(generate_mrn())
    .fetch_one(conn)
    .await?;

    Ok(rec)
}

pub async fn find_by_id(conn: &mut PgConnection, id: Uuid) -> Result<Option<Patient>, sqlx::Error> {
    let rec = sqlx::query_as("SELECT * FROM patients WHERE id = $1")
        .bind(id)
        .fetch_optional(conn)
        .await?;

    Ok(rec)
}

pub async fn find_all(conn: &mut PgConnection) -> Result<Vec<Patient>, sqlx::Error> {
    let rec = sqlx::query_as::<_, Patient>("SELECT * FROM patients ORDER BY created_at DESC")
        .fetch_all(conn)
        .await?;
    Ok(rec)
}
