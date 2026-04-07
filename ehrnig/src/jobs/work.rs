use crate::utility::constants;
use deadpool_lapin::Pool;
use futures_util::StreamExt;
use lapin::{options::*, types::FieldTable};

pub async fn start_workers(pool: Pool) {
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
            constants::EVENT_PATIENT_CREATED,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await
        .expect("Worker: Could not declare queue");

    // 3. Start the Consumer
    let mut consumer = channel
        .basic_consume(
            constants::EVENT_PATIENT_CREATED,
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
                let data = String::from_utf8_lossy(&delivery.data);

                tracing::info!("JOB Received message: {}", data);

                // --- Business Logic Happens Here ---

                // 5. Acknowledge the message
                if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                    tracing::error!("Worker: Failed to ack message: {:?}", e);
                }
            }
            Err(e) => {
                tracing::error!("Worker: Error receiving message: {:?}", e);
            }
        }
    }
}
