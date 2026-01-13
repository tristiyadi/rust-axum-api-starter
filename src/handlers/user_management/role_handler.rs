use axum::{Extension, Json, http::StatusCode};
use serde_json::{Value, json};
use sqlx::MySqlPool;

// import util response API
use crate::utils::response::ApiResponse;

pub async fn index(Extension(db): Extension<MySqlPool>) -> (StatusCode, Json<ApiResponse<Value>>) {
    // Ambil seluruh data role
    let roles = match sqlx::query_as::<_, crate::models::role::Role>(
        r#"
        SELECT roles_id, name, display_name, description, status, created_at, updated_at
        FROM roles
        WHERE status = 1
        ORDER BY roles_id ASC
        "#,
    )
    .fetch_all(&db)
    .await
    {
        Ok(roles) => roles,
        Err(e) => {
            eprintln!("Database error: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error("Gagal mengambil data role")),
            );
        }
    };

    (
        StatusCode::OK,
        Json(ApiResponse::success("List role", json!(roles))),
    )
}
