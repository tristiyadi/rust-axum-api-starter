use axum::{
    Router,
    routing::{get, post, put, delete},
    middleware,
};

// import handler user
use crate::handlers::user_handler::{
    index,
    store,
    show,
    update,
    destroy,
};

// import middleware auth
use crate::middlewares::auth_middleware::auth;

pub fn user_routes() -> Router {
    Router::new()
        .route("/api/users", get(index))
        .route("/api/users", post(store))
        .route("/api/users/{id}", get(show))
        .route("/api/users/{id}", post(update))
        .route("/api/users/{id}", put(update))
        .route("/api/users/{id}", delete(destroy))

        
        // Semua route di atas WAJIB login
        .layer(middleware::from_fn(auth))
}
