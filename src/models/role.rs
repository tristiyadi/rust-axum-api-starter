use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct Role {
    pub roles_id: u64,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub status: bool,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
