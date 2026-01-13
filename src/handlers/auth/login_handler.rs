use axum::{Extension, Json, http::StatusCode};
use bcrypt::verify;
use serde_json::{Value, json};
use sqlx::MySqlPool;
use std::collections::HashMap;
use validator::Validate;

// import model user
use crate::models::user::User;

// import schema request dan response login
use crate::schemas::login_schema::{LoginRequest, LoginResponse, UserResponse};

// import util jwt generate token dan response API
use crate::utils::{jwt::generate_token, response::ApiResponse};

pub async fn login(
    Extension(db): Extension<MySqlPool>,
    Json(payload): Json<LoginRequest>,
) -> (StatusCode, Json<ApiResponse<Value>>) {
    // Validasi Request
    if let Err(errors) = payload.validate() {
        let mut field_errors: HashMap<String, Vec<String>> = HashMap::new();

        // kumpulkan semua error dari validasi
        for (field, errors) in errors.field_errors() {
            let messages = errors
                .iter()
                .filter_map(|e| e.message.as_ref())
                .map(|m| m.to_string())
                .collect::<Vec<String>>();

            field_errors.insert(field.to_string(), messages);
        }

        return (
            // kirim response 422 Unprocessable Entity
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ApiResponse {
                status: false,
                message: "Validasi Gagal".to_string(),
                data: Some(json!(field_errors)),
            }),
        );
    }

    // get user berdasarkan email
    let user = match sqlx::query_as::<_, User>(
        r#"
        SELECT * 
        FROM users 
        WHERE email = ?
        "#,
    )
    .bind(&payload.email)
    .fetch_one(&db)
    .await
    {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return (
                // kirim response 401 Unauthorized
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error("Email atau Password Anda Salah")),
            );
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            return (
                // kirim response 500 Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Terjadi kesalahan sistem")),
            );
        }
    };

    // Check if user status is active
    if let Some(ref status) = user.status {
        if status != "active" {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error("Akun Anda tidak aktif")),
            );
        }
    }

    // Verifikasi Password Dengan Bcrypt
    let password_hash = user.password.unwrap_or_default();
    match verify(payload.password, &password_hash) {
        Ok(true) => {
            // generate token JWT dengan role_id
            match generate_token(user.id, user.role_id) {
                Ok(token) => {
                    let response = LoginResponse {
                        user: UserResponse {
                            id: user.id,
                            name: user.name,
                            email: user.email,
                            uid: user.uid,
                            role_id: user.role_id,
                            username: user.username,
                            status: user.status,
                        },
                        token,
                    };

                    (
                        // kirim response 200 OK
                        StatusCode::OK,
                        Json(ApiResponse::success("Login Berhasil", json!(response))),
                    )
                }
                Err(e) => {
                    eprintln!("JWT generation error: {:?}", e);
                    (
                        // kirim response 500 Internal Server Error
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse::error("Gagal membuat token")),
                    )
                }
            }
        }
        Ok(false) => (
            // kirim response 401 Unauthorized
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error("Email atau Password Anda Salah")),
        ),
        Err(_) => (
            // kirim response 500 Internal Server Error
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Gagal memverifikasi password")),
        ),
    }
}
