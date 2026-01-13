use axum::{
    Router, middleware,
    routing::{delete, get, post, put},
};

// import handler user dari module user_management
use crate::handlers::user_management::user_handler::{destroy, index, show, store, update};

// import middleware auth
use crate::middlewares::auth_middleware::auth;

pub fn user_routes() -> Router {
    Router::new()
        .route("/api/users", get(index))
        .route("/api/users", post(store))
        .route("/api/users/{uid}", get(show))
        .route("/api/users/{uid}", post(update))
        .route("/api/users/{uid}", put(update))
        .route("/api/users/{uid}", delete(destroy))
        // Semua route di atas WAJIB login
        .layer(middleware::from_fn(auth))
}
