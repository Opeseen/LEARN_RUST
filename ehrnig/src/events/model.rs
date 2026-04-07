use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue; // Using sqlx's alias for serde_json::Value
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct OutboxEvent {
    pub id: Uuid,
    pub event_type: String,
    pub payload: JsonValue,
    pub organization_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}
