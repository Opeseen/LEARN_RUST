use anyhow::Result;
use deadpool_lapin::Pool;
use lapin::{BasicProperties, options::*, types::FieldTable};
use serde::Serialize;

pub async fn publish_event<T: Serialize>(pool: &Pool, queue_name: &str, payload: &T) -> Result<()> {
    let conn = pool.get().await?;
    let channel = conn.create_channel().await?;

    channel
        .queue_declare(
            queue_name,
            QueueDeclareOptions::default(),
            FieldTable::default(),
        )
        .await?;

    // Convert struct to JSON bytes
    let body = serde_json::to_vec(payload)?;

    channel
        .basic_publish(
            "",
            queue_name,
            BasicPublishOptions::default(),
            &body,
            // Making the message persistent (survives RabbitMQ restart)
            BasicProperties::default().with_delivery_mode(2),
        )
        .await?;

    Ok(())
}
