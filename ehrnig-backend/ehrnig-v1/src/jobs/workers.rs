use crate::events::EHRJob;
use crate::infrastructures::EmailService;
use crate::utility::constants;
use deadpool_lapin::Pool;
use futures_util::StreamExt;
use lapin::{options::*, types::FieldTable};

pub async fn start_workers(pool: Pool, email_service: EmailService) {
    // 1. Get a connection from the pool
    let conn = pool
        .get()
        .await
        .expect("Worker: Could not get MQ connection");

    let channel = conn
        .create_channel()
        .await
        .expect("Worker: Could not create channel");

    // 2. Declare the queue
    channel
        .queue_declare(
            constants::JOB_QUEUE_NAME,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Worker: Could not declare queue");

    // 3. Start the Consumer
    let mut consumer = channel
        .basic_consume(
            constants::JOB_QUEUE_NAME,
            "ehr_worker",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Worker: Could not start consumer");

    tracing::info!("Background Workers are active and listening for events...");

    // 4. The Event Loop
    while let Some(delivery_result) = consumer.next().await {
        match delivery_result {
            Ok(delivery) => {
                // pass the bytes directly into the enum
                let job = serde_json::from_slice::<EHRJob>(&delivery.data);
                match job {
                    Ok(job) => {
                        // need to await because it's an async
                        if let Err(e) = handle_job(job, &email_service).await {
                            tracing::error!("Worker: Job failed: {:?}", e);
                            // reject with requeue, to retry later
                            let _ = delivery.reject(BasicRejectOptions { requeue: true }).await;
                        } else {
                            // acknowledge on success
                            let _ = delivery.ack(BasicAckOptions::default()).await;
                        }
                    }
                    Err(e) => {
                        tracing::error!("Worker: Failed to parse JSON: {:?}", e);
                        // don't requeue again
                        let _ = delivery.reject(BasicRejectOptions { requeue: false }).await;
                    }
                }
            }
            Err(e) => {
                tracing::error!("Worker: Error receiving message: {:?}", e);
            }
        }
    }
}

async fn handle_job(job: EHRJob, email_service: &EmailService) -> anyhow::Result<()> {
    match job {
        EHRJob::SendEmail(payload) => {
            tracing::info!(
                "Sending email to {} for Org {}",
                payload.recipient_name,
                payload.organization_id
            );
            // Call the actual email service here
            email_service
                .send_welcome_email("admin@example.com", &payload.recipient_name)
                .await?;
            Ok(())
        }

        _ => Ok(()),
    }
}
