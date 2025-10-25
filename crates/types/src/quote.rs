use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a row in the `quote` table
#[derive(Debug, Serialize, Deserialize)]
pub struct Quote {
    pub id: Uuid,
    pub author: String,
    pub quote: String
}
