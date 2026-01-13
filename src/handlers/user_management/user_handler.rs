use axum::{Extension, Json, extract::Path, http::StatusCode};
use bcrypt::hash;
use serde_json::{Value, json};
use sqlx::MySqlPool;
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

// import util response API
use crate::utils::response::ApiResponse;

// import schema request dan response user
use crate::schemas::user_schema::{UserResponse, UserStoreRequest, UserUpdateRequest};

pub async fn index(Extension(db): Extension<MySqlPool>) -> (StatusCode, Json<ApiResponse<Value>>) {
    // Ambil seluruh data user
    let users = match sqlx::query_as::<_, UserResponse>(
        r#"
        SELECT id, name, email, email_verified_at, status, uid, role_id, username, users_token, created_at, updated_at
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
        Json(ApiResponse::success("List user", json!(users))),
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
                // kirim response 500 Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Gagal mengenkripsi password")),
            );
        }
    };

    // Generate UID untuk user
    let uid = Uuid::new_v4().to_string();

    // Default status untuk user baru
    let status = "active";

    // Role ID dari request atau default 2
    let role_id = payload.role_id.unwrap_or(2);

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

            // Ambil data user berdasarkan id
            let user = sqlx::query_as::<_, UserResponse>(
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
                    let response = UserResponse {
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
                        Json(ApiResponse::success(
                            "User berhasil ditambahkan",
                            json!(response),
                        )),
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
                    Json(ApiResponse::error("Gagal menambahkan user")),
                )
            }
        }
    }
}

pub async fn show(
    Path(uid): Path<String>,
    Extension(db): Extension<MySqlPool>,
) -> (StatusCode, Json<ApiResponse<Value>>) {
    // Ambil data user berdasarkan UID
    let user = match sqlx::query_as::<_, UserResponse>(
        r#"
        SELECT id, name, email, email_verified_at, status, uid, role_id, username, users_token, created_at, updated_at
        FROM users
        
        WHERE uid = ?
        "#,
    )
    .bind(&uid)
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

    (
        // kirim response 200 OK
        StatusCode::OK,
        Json(ApiResponse::success("Detail user", json!(user))),
    )
}

pub async fn update(
    Path(uid): Path<String>,
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

        // Cek konfirmasi password
        if password != &payload.password_confirmation {
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
    }

    // Cek user exist
    let user_exist = match sqlx::query_as::<_, UserResponse>(
        "SELECT id, name, email, email_verified_at, status, uid, role_id, username, users_token, created_at, updated_at FROM users WHERE uid = ?"
    )
    .bind(&uid)
    .fetch_one(&db)
    .await
    {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error("User tidak ditemukan")),
            );
        }
        Err(e) => {
            eprintln!("Database error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Terjadi kesalahan sistem")),
            );
        }
    };

    // Cek email unique (kecuali diri sendiri)
    let email_exists = sqlx::query("SELECT id FROM users WHERE email = ? AND id != ?")
        .bind(&payload.email)
        .bind(user_exist.id)
        .fetch_optional(&db)
        .await;

    if let Ok(Some(_)) = email_exists {
        return (
            StatusCode::CONFLICT,
            Json(ApiResponse::error("Email sudah terdaftar")),
        );
    }

    // Build update query based on what fields are provided
    let mut update_parts = vec!["name = ?", "email = ?"];
    let mut values: Vec<Value> = vec![json!(payload.name), json!(payload.email)];

    if let Some(password) = &payload.password {
        if !password.is_empty() {
            let hashed = match hash(password, 10) {
                Ok(h) => h,
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse::error("Gagal mengenkripsi password")),
                    );
                }
            };
            update_parts.push("password = ?");
            values.push(json!(hashed));
        }
    }

    if let Some(username) = &payload.username {
        update_parts.push("username = ?");
        values.push(json!(username));
    }

    if let Some(role_id) = payload.role_id {
        update_parts.push("role_id = ?");
        values.push(json!(role_id));
    }

    if let Some(status) = &payload.status {
        update_parts.push("status = ?");
        values.push(json!(status));
    }

    let update_query = format!("UPDATE users SET {} WHERE uid = ?", update_parts.join(", "));

    // Bind all values
    let mut query = sqlx::query(&update_query);
    for val in values {
        if val.is_string() {
            query = query.bind(val.as_str().unwrap().to_string());
        } else if val.is_number() {
            query = query.bind(val.as_u64().unwrap() as u32);
        }
    }
    // Bind the UID for WHERE clause
    query = query.bind(&uid);

    if let Err(e) = query.execute(&db).await {
        eprintln!("Database error updating user: {}", e);
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error("Gagal memperbarui data user")),
        );
    }

    // Ambil data terbaru
    let user = match sqlx::query_as::<_, UserResponse>(
        r#"
        SELECT id, name, email, email_verified_at, status, uid, role_id, username, users_token, created_at, updated_at
        FROM users
        WHERE uid = ?
        "#,
    )
    .bind(&uid)
    .fetch_one(&db)
    .await {
        Ok(u) => u,
        Err(e) => {
            eprintln!("Database error fetching updated user: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Gagal mengambil data terbaru user")),
            );
        }
    };

    (
        // kirim response 200 OK
        StatusCode::OK,
        Json(ApiResponse::success(
            "User berhasil diperbarui",
            json!(user),
        )),
    )
}

pub async fn destroy(
    Path(uid): Path<String>,
    Extension(db): Extension<MySqlPool>,
) -> (StatusCode, Json<ApiResponse<Value>>) {
    // Cek user exist
    let user = match sqlx::query_as::<_, UserResponse>(
        "SELECT id, name, email, email_verified_at, status, uid, role_id, username, users_token, created_at, updated_at FROM users WHERE uid = ?"
    )
    .bind(&uid)
    .fetch_one(&db)
    .await
    {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return (
                // kirim response 404 Not Found
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error("User tidak ditemukan")),
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

    // Hapus user dari database
    let result = sqlx::query("DELETE FROM users WHERE uid = ?")
        .bind(&user.uid)
        .execute(&db)
        .await;

    match result {
        Ok(_) => (
            // kirim response 200 OK
            StatusCode::OK,
            Json(ApiResponse::success("User berhasil dihapus", json!(null))),
        ),
        Err(e) => {
            eprintln!("Database error: {}", e);
            (
                // kirim response 500 Internal Server Error
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Gagal menghapus user")),
            )
        }
    }
}
