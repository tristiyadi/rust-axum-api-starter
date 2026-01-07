use axum::{
    Extension,
    Json,
    http::StatusCode,
    extract::Path,
};
use sqlx::MySqlPool;
use bcrypt::hash;
use validator::Validate;
use std::collections::HashMap;
use serde_json::{json, Value};

// import model user
use crate::models::user::User;

// import util response API
use crate::utils::response::ApiResponse;

// import schema request dan response user
use crate::schemas::user_schema::{
    UserStoreRequest,
    UserUpdateRequest,
    UserResponse,
};

pub async fn index(
    Extension(db): Extension<MySqlPool>,
) -> (StatusCode, Json<ApiResponse<Value>>) {

    // Ambil seluruh data user
    let users = match sqlx::query_as!(
        User,
        r#"
        SELECT id, name, email, created_at, updated_at
        FROM users
        ORDER BY id DESC
        "#
    )
    .fetch_all(&db)
    .await
    {
        Ok(users) => users,
        Err(e) => {
            eprintln!("Database error: {}", e);
            return (
                // kirim response 500 Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Gagal mengambil data user",
                )),
            );
        }
    };

    (
        // kirim response 200 OK
        StatusCode::OK,
        Json(ApiResponse::success(
            "List user",
            json!(users),
        )),
    )
}

pub async fn store(
    Extension(db): Extension<MySqlPool>,
    Json(payload): Json<UserStoreRequest>,
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

    // Hash Password Dengan Bcrypt
    let password = match hash(payload.password, 10) {
        Ok(hashed) => hashed,
        Err(_) => {
            return (
                // kirim response 500 Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Gagal mengenkripsi password",
                )),
            );
        }
    };

    // Insert Data User ke Database
    let result = sqlx::query!(
        "INSERT INTO users (name, email, password) VALUES (?, ?, ?)",
        payload.name,
        payload.email,
        password
    )
    .execute(&db)
    .await;

    match result {
        Ok(result) => {

            // get id user yang baru saja dibuat
            let user_id = result.last_insert_id() as i64;

            // Ambil data user berdasarkan id
            let user = sqlx::query!(
                r#"
                SELECT id, name, email, created_at, updated_at
                FROM users
                WHERE id = ?
                "#,
                user_id
            )
            .fetch_one(&db)
            .await;

            match user {
                Ok(user) => {
                    let response = UserResponse {
                        id: user.id,
                        name: user.name,
                        email: user.email,
                        created_at: user.created_at,
                        updated_at: user.updated_at,
                    };

                    (
                        // kirim response 201 Created
                        StatusCode::CREATED,
                        Json(ApiResponse::success(
                            "User berhasil ditambahkan",
                            json!(response),
                        )),
                    )
                }
                Err(_) => (
                    // kirim response 500 Internal Server Error
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::error(
                        "Gagal mengambil data user",
                    )),
                ),
            }
        }
        Err(e) => {
            if e.to_string().contains("Duplicate entry") {
                (
                    // kirim response 409 Conflict
                    StatusCode::CONFLICT,
                    Json(ApiResponse::error(
                        "Email sudah terdaftar",
                    )),
                )
            } else {
                (
                    // kirim response 500 Internal Server Error
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiResponse::error(
                        "Gagal menambahkan user",
                    )),
                )
            }
        }
    }
}
pub async fn show(
    Path(id): Path<i64>,
    Extension(db): Extension<MySqlPool>,
) -> (StatusCode, Json<ApiResponse<Value>>) {

    // Ambil data user berdasarkan ID
    let user = match sqlx::query!(
        r#"
        SELECT id, name, email, created_at, updated_at
        FROM users
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(&db)
    .await
    {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return (
                // kirim response 404 Not Found
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error(
                    "User tidak ditemukan",
                )),
            );
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            return (
                // kirim response 500 Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Gagal mengambil data user",
                )),
            );
        }
    };

    let response = UserResponse {
        id: user.id,
        name: user.name,
        email: user.email,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    (
        // kirim response 200 OK
        StatusCode::OK,
        Json(ApiResponse::success(
            "Detail user",
            json!(response),
        )),
    )
}

pub async fn update(
    Path(id): Path<i64>,
    Extension(db): Extension<MySqlPool>,
    Json(payload): Json<UserUpdateRequest>,
) -> (StatusCode, Json<ApiResponse<Value>>) {

    // Validasi dasar (name & email)
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

    // Validasi password opsional
    if let Some(password) = &payload.password {
        if !password.is_empty() && password.len() < 6 {
            let mut errors = HashMap::new();
            errors.insert(
                "password".to_string(),
                vec!["Password minimal 6 karakter".to_string()],
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
    }

    // Cek user exist
    let user_exist = match sqlx::query!(
        "SELECT id FROM users WHERE id = ?",
        id
    )
    .fetch_one(&db)
    .await
    {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error(
                    "User tidak ditemukan",
                )),
            );
        }
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Terjadi kesalahan sistem",
                )),
            );
        }
    };

    // Cek email unique (kecuali diri sendiri)
    let email_exists = sqlx::query!(
        "SELECT id FROM users WHERE email = ? AND id != ?",
        payload.email,
        user_exist.id
    )
    .fetch_optional(&db)
    .await;

    if let Ok(Some(_)) = email_exists {
        return (
            StatusCode::CONFLICT,
            Json(ApiResponse::error(
                "Email sudah terdaftar",
            )),
        );
    }

    // Update user
    let result = match &payload.password {
        Some(password) if !password.is_empty() => {
            
            // Hash password Dengan Bcrypt
            let hashed = match hash(password, 10) {
                Ok(h) => h,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse::error(
                            "Gagal mengenkripsi password",
                        )),
                    );
                }
            };

            // Update user dengan password
            sqlx::query!(
                "UPDATE users SET name = ?, email = ?, password = ? WHERE id = ?",
                payload.name,
                payload.email,
                hashed,
                id
            )
            .execute(&db)
            .await
        }
        _ => {

            // Update user tanpa password
            sqlx::query!(
                "UPDATE users SET name = ?, email = ? WHERE id = ?",
                payload.name,
                payload.email,
                id
            )
            .execute(&db)
            .await
        }
    };

    if let Err(_) = result {
        return (

            // kirim response 500 Internal Server Error
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(
                "Gagal memperbarui data user",
            )),
        );
    }

    // Ambil data terbaru
    let user = sqlx::query!(
        r#"
        SELECT id, name, email, created_at, updated_at
        FROM users
        WHERE id = ?
        "#,
        id
    )
    .fetch_one(&db)
    .await
    .unwrap();

    let response = UserResponse {
        id: user.id,
        name: user.name,
        email: user.email,
        created_at: user.created_at,
        updated_at: user.updated_at,
    };

    (
        // kirim response 200 OK
        StatusCode::OK,
        Json(ApiResponse::success(
            "User berhasil diperbarui",
            json!(response),
        )),
    )
}
pub async fn destroy(
    Path(id): Path<i64>,
    Extension(db): Extension<MySqlPool>,
) -> (StatusCode, Json<ApiResponse<Value>>) {

    // Cek user exist
    let user = match sqlx::query!(
        "SELECT id FROM users WHERE id = ?",
        id
    )
    .fetch_one(&db)
    .await
    {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return (
                // kirim response 404 Not Found
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error(
                    "User tidak ditemukan",
                )),
            );
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            return (
                // kirim response 500 Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Terjadi kesalahan sistem",
                )),
            );
        }
    };

    // Hapus user dari database
    let result = sqlx::query!(
        "DELETE FROM users WHERE id = ?",
        user.id
    )
    .execute(&db)
    .await;

    match result {
        Ok(_) => (
            // kirim response 200 OK
            StatusCode::OK,
            Json(ApiResponse::success(
                "User berhasil dihapus",
                json!(null),
            )),
        ),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                // kirim response 500 Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(
                    "Gagal menghapus user",
                )),
            )
        }
    }
}

