use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Represents a row in the `quote` table
#[derive(Debug)]
pub struct Quote {
    pub id: Uuid,
    pub author: String,
    pub quote: String,
    pub created_at: DateTime<Utc>,
}
