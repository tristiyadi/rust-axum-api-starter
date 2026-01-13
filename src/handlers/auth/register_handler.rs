use axum::{Extension, Json, http::StatusCode};
use bcrypt::hash;
use serde_json::{Value, json};
use sqlx::MySqlPool;
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

// import schema request dan response register
use crate::schemas::register_schema::{RegisterRequest, RegisterResponse};

// import util response API
use crate::utils::response::ApiResponse;

pub async fn register(
    Extension(db): Extension<MySqlPool>,
    Json(payload): Json<RegisterRequest>,
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

    // Cek konfirmasi password
    if payload.password != payload.password_confirmation {
        let mut errors = HashMap::new();
        errors.insert(
            "password_confirmation".to_string(),
            vec!["Konfirmasi password tidak cocok".to_string()],
        );

        return (
            StatusCode::UNPROCESSABLE_ENTITY,
            Json(ApiResponse {
                status: false,
                message: "Validasi Gagal".to_string(),
                data: Some(json!(errors)),
            }),
        );
    }

    // Hash Password Dengan Bcrypt
    let password = match hash(payload.password, 10) {
        Ok(hashed) => hashed,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Gagal mengenkripsi password")),
            );
        }
    };

    // Generate UID untuk user
    let uid = Uuid::new_v4().to_string();

    // Default status untuk user baru
    let status = "active";

    // Default role_id
    let role_id: u32 = 2;

    // Insert Data User ke Database
    let result = sqlx::query(
        r#"
        INSERT INTO users (name, email, password, uid, status, role_id, username) 
        VALUES (?, ?, ?, ?, ?, ?, ?)
        "#,
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(&password)
    .bind(&uid)
    .bind(&status)
    .bind(&role_id)
    .bind(payload.username.as_ref().unwrap_or(&payload.email))
    .execute(&db)
    .await;

    match result {
        Ok(result) => {
            // get id user yang baru saja dibuat
            let user_id = result.last_insert_id() as u32;
            println!("User ID: {}", user_id);
            // Ambil data user berdasarkan ID
            let user = sqlx::query_as::<_, RegisterResponse>(
                r#"
                SELECT id, name, email, email_verified_at, status, uid, role_id, username, users_token, created_at, updated_at
                FROM users
                WHERE id = ?
                "#,
            )
            .bind(user_id)
            .fetch_one(&db)
            .await;

            match user {
                Ok(user) => {
                    let response = RegisterResponse {
                        id: user.id,
                        name: user.name,
                        email: user.email,
                        email_verified_at: user.email_verified_at,
                        status: user.status,
                        uid: user.uid,
                        role_id: user.role_id,
                        username: user.username,
                        users_token: user.users_token,
                        created_at: user.created_at,
                        updated_at: user.updated_at,
                    };

                    (
                        // kirim response 201 Created
                        StatusCode::CREATED,
                        Json(ApiResponse::success("Register Berhasil!", json!(response))),
                    )
                }
                Err(e) => {
                    eprintln!("Database error fetching user: {}", e);
                    (
                        // kirim response 500 Internal Server Error
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse::error("Gagal mengambil data user")),
                    )
                }
            }
        }
        Err(e) => {
            if e.to_string().contains("Duplicate entry") {
                (
                    // kirim response 409 Conflict
                    StatusCode::CONFLICT,
                    Json(ApiResponse::error("Email sudah terdaftar")),
                )
            } else {
                (
                    // kirim response 500 Internal Server Error
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::error("Register Gagal!")),
                )
            }
        }
    }
}
