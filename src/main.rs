use axum::{Router, Extension};
use dotenvy::dotenv;
use tower_http::cors::{CorsLayer, Any};

mod config;
mod models;
mod handlers;
mod routes;
mod schemas;
mod utils;
mod middlewares;


#[tokio::main]
async fn main() {
    
    // Load environment variables from .env file
    dotenv().ok();

    // koneksi ke database
    let db = config::database::connect().await;

    // Konfigurasi CORS
    let cors = CorsLayer::new()
        .allow_origin(Any) // Izinkan semua origin
        .allow_methods(Any) // Izinkan semua method (GET, POST, dll)
        .allow_headers(Any);

    // Buat router dasar
    let app = Router::new()
        .merge(routes::auth_routes::auth_routes())
        .merge(routes::user_routes::user_routes())
        .layer(Extension(db))
        .layer(cors);

    // Ambil port & host dari environment variable, default 3000
    let port = std::env::var("APP_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3000);
    let host = std::env::var("APP_HOST")
        .ok()
        .unwrap_or_else(|| "localhost".into());

    // Alamat server
    let addr = format!("{}:{}", host, port);
    
    // Tampilkan alamat server di console
    println!("Server running on http://{}", addr);
    
    // Jalankan server
    axum::serve(
        tokio::net::TcpListener::bind(&addr).await.unwrap(),
        app
    ).await.unwrap();
}
