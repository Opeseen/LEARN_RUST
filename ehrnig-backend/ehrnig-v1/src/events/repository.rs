use serde_json::Value;
use sqlx::PgConnection;
use uuid::Uuid;

pub async fn create_event(
    conn: &mut PgConnection,
    org_id: Uuid,
    event_type: &str,
    data: Value,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO outbox_events (event_type, payload, organization_id)
    VALUES ($1, $2, $3)",
    )
    .bind(event_type)
    .bind(data)
    .bind(org_id)
    .execute(&mut *conn)
    .await?;

    // trigger notification for background workers
    notify_outbox(conn).await?;

    Ok(())
}

// background workers notification
async fn notify_outbox(conn: &mut PgConnection) -> Result<(), sqlx::Error> {
    sqlx::query("NOTIFY outbox_updated").execute(conn).await?;

    Ok(())
}
