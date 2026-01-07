use axum::{
    Extension,
    Json,
    http::StatusCode,
};
use sqlx::MySqlPool;
use bcrypt::verify;
use validator::Validate;
use std::collections::HashMap;
use serde_json::{json, Value};

// import schema request dan response login
use crate::schemas::login_schema::{
    LoginRequest,
    LoginResponse,
    UserResponse,
};

// import util jwt generate token dan response API
use crate::utils::{
    jwt::generate_token,
    response::ApiResponse,
};

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
    let user = match sqlx::query!(
        "SELECT id, name, email, password FROM users WHERE email = ?",
        payload.email
    )
    .fetch_one(&db)
    .await
    {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            return (

                // kirim response 401 Unauthorized
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(
                    "Email atau Password Anda Salah",
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

    // Verifikasi Password Dengan Bcrypt
    match verify(payload.password, &user.password) {
        Ok(true) => {
            
            // generate token JWT
            match generate_token(user.id) {
                Ok(token) => {
                    let response = LoginResponse {
                        user: UserResponse {
                            id: user.id,
                            name: user.name,
                            email: user.email,
                        },
                        token,
                    };

                    (
                        // kirim response 200 OK
                        StatusCode::OK,
                        Json(ApiResponse::success(
                            "Login Berhasil",
                            json!(response),
                        )),
                    )
                }
                Err(e) => {
                    eprintln!("JWT generation error: {:?}", e);
                    (
                        // kirim response 500 Internal Server Error
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse::error(
                            "Gagal membuat token",
                        )),
                    )
                }
            }
        }
        Ok(false) => (

            // kirim response 401 Unauthorized
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(
                "Email atau Password Anda Salah",
            )),
        ),
        Err(_) => (

            // kirim response 500 Internal Server Error
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(
                "Gagal memverifikasi password",
            )),
        ),
    }
}
