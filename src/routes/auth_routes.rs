use axum::{Router, routing::post};

// import handler register
use crate::handlers::register_handler::register;
// import handler login
use crate::handlers::login_handler::login;

// fungsi untuk mengatur route autentikasi
pub fn auth_routes() -> Router {
    Router::new()
        // route auth list
        .route("/api/register", post(register))
        .route("/api/login", post(login))
}
