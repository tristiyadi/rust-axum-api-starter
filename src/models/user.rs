use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, sqlx::FromRow)]
pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub password: Option<String>,
    pub status: Option<String>,
    pub uid: Option<String>,
    pub role_id: u32,
    pub username: Option<String>,
    pub remember_token: Option<String>,
    pub users_token: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
