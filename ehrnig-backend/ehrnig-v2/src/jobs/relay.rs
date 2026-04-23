// This sees the new inserted DB row event -> Publishes to RabbitMQ -> Updates DB row to "Processed"

use crate::events::{EHRJob, EmailPayload, OutboxEvent};
use crate::jobs::publish_event;
use crate::patients::CreatePatientDto;
use crate::utility::constants;
use deadpool_lapin::Pool as MqPool;
use sqlx::PgPool as DbPool;
use sqlx::postgres::PgListener;
use uuid::Uuid;

// this listen for postgres NOTIFY signal
pub async fn start_relay_worker(db: DbPool, mq: MqPool) {
    let mut listener = PgListener::connect_with(&db)
        .await
        .expect("Listener failed");
    listener
        .listen("outbox_updated")
        .await
        .expect("Listen command failed");

    tracing::info!("Outbox Relay is active and waiting for signals...");

    // INFO: Run once on startup in case there are old unprocessed rows
    if let Err(e) = run_relay_process(&db, &mq).await {
        tracing::error!("Startup relay error: {:?}", e);
    }

    loop {
        // This waits here without using CPU until 'NOTIFY outbox_updated' is called
        while let Ok(_notification) = listener.recv().await {
            if let Err(e) = run_relay_process(&db, &mq).await {
                tracing::error!("Relay process error: {:?}", e);
            }
        }
    }
}

async fn run_relay_process(db: &DbPool, mq: &MqPool) -> anyhow::Result<()> {
    // Fetch all unprocessed events in the order they happened
    let events = sqlx::query_as!(
        OutboxEvent,
        "SELECT * FROM outbox_events WHERE processed_at IS NULL ORDER BY created_at ASC",
    )
    .fetch_all(db)
    .await?;

    for ev in events {
        // Convert specific DB event to generic Job Enum
        match map_event_to_job(&ev) {
            Ok(Some(job)) => {
                // Found mapping, try to publish to queue
                if let Ok(_) = publish_event(mq, constants::JOB_QUEUE_NAME, &job).await {
                    mark_as_processed(db, ev.id).await?;
                    tracing::info!("Relayed event: {}", ev.event_type);
                }
            }

            Ok(None) => {
                // Mapping was FOUND but explicitly ignored so, we mark it processed so we don't look at it again.
                mark_as_processed(db, ev.id).await?;
            }
            _ => continue, // skip to jump to the next event in the loop
        };
    }

    Ok(())
}

async fn mark_as_processed(db: &DbPool, event_id: Uuid) -> anyhow::Result<()> {
    sqlx::query!(
        "UPDATE outbox_events SET processed_at = NOW() WHERE id = $1",
        event_id,
    )
    .execute(db)
    .await?;

    Ok(())
}

fn map_event_to_job(ev: &OutboxEvent) -> anyhow::Result<Option<EHRJob>> {
    match ev.event_type.as_str() {
        constants::EVENT_PATIENT_CREATED => {
            let p: CreatePatientDto = serde_json::from_value(ev.payload.clone())?;
            Ok(Some(EHRJob::SendEmail(EmailPayload {
                recipient_name: format!("{} {}", p.first_name, p.last_name),
                organization_id: ev.organization_id,
            })))
        }

        _ => {
            tracing::warn!("No job mapping found for event: {}", ev.event_type);
            Ok(None)
        }
    }
}
