use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct UserStoreRequest {
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    pub name: String,

    #[validate(email(message = "Email tidak valid"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password minimal 6 karakter"))]
    pub password: String,

    #[validate(length(min = 6, message = "Confirmation Password minimal 6 karakter"))]
    pub password_confirmation: String,

    #[validate(length(min = 3, message = "Username minimal 3 karakter"))]
    pub username: Option<String>,

    pub role_id: Option<u32>,
}

#[derive(Deserialize, Validate)]
pub struct UserUpdateRequest {
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    pub name: String,

    #[validate(email(message = "Email tidak valid"))]
    pub email: String,
    pub password: Option<String>,
    pub password_confirmation: String,
    pub username: Option<String>,
    pub role_id: Option<u32>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserResponse {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub email_verified_at: Option<DateTime<Utc>>,
    pub status: Option<String>,
    pub uid: Option<String>,
    pub role_id: u32,
    pub username: Option<String>,
    pub users_token: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
