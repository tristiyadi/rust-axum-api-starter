use axum::{Router, routing::post};

// import handler register
use crate::handlers::register_handler::register;

// fungsi untuk mengatur route autentikasi
pub fn auth_routes() -> Router {
    Router::new()
        // route untuk register
        .route("/api/register", post(register))
}
