use uuid::Uuid;

/// Represents a row in the `quote` table
#[derive(Debug)]
pub struct Quote {
    pub id: Uuid,
    pub author: String,
    pub quote: String
}
