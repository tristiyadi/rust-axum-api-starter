use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Email tidak valid"))]
    pub email: String,

    #[validate(length(min = 6, message = "Password minimal 6 karakter"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserResponse {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub uid: Option<String>,
    pub role_id: u32,
    pub username: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub token: String,
}
