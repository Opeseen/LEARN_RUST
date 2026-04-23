use super::config::Config;
use sqlx::PgConnection;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use uuid::Uuid;

pub async fn create_pool(url: String) -> Result<PgPool, sqlx::Error> {
    tracing::info!("Connecting to database...");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&url)
        .await?;

    Ok(pool)
}

pub async fn set_tenant_context(conn: &mut PgConnection, org_id: Uuid) -> Result<(), sqlx::Error> {
    // The `true` flag means SET LOCAL — it resets automatically when the
    // transaction commits or rolls back, so pooled connections stay clean.
    sqlx::query("SELECT set_config('app.current_organization_id', $1, true)")
        .bind(org_id.to_string())
        .execute(conn)
        .await?;
    Ok(())
}
