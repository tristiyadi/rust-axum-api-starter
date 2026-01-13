use axum::{Router, middleware, routing::get};

// import handler role dari module role
use crate::handlers::user_management::role_handler::index;

// import middleware auth
use crate::middlewares::auth_middleware::auth;

pub fn role_routes() -> Router {
    Router::new()
        .route("/api/roles", get(index))
        // Semua route di atas WAJIB login
        .layer(middleware::from_fn(auth))
}
