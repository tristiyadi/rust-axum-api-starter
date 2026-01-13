use axum::{Router, routing::post};

// import handler register dan login dari module auth
use crate::handlers::auth::login_handler::login;
use crate::handlers::auth::register_handler::register;

// fungsi untuk mengatur route autentikasi
pub fn auth_routes() -> Router {
    Router::new()
        // route auth list
        .route("/api/register", post(register))
        .route("/api/login", post(login))
}
