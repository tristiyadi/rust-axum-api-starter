use axum::{Router, Extension};
use dotenvy::dotenv;
use std::net::SocketAddr;

mod config;
mod models;
mod handlers;
mod routes;
mod schemas;
mod utils;

#[tokio::main]
async fn main() {
    
    // Load environment variables from .env file
    dotenv().ok();

    // koneksi ke database
    let db = config::database::connect().await;


    // Buat router dasar
    let app = Router::new()
        .merge(routes::auth_routes::auth_routes())
        .layer(Extension(db));

    // Ambil port & host dari environment variable, default 3000
    let port = std::env::var("APP_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3001);
    let host = std::env::var("APP_HOST")
        .ok()
        .unwrap_or_else(|| "127.0.0.1".into());

    // Alamat server
    let addr = SocketAddr::from((host.parse::<std::net::IpAddr>().unwrap(), port));
    
    // Tampilkan alamat server di console
    println!("Server running on http://{}", addr);
    
    // Jalankan server
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    ).await.unwrap();
}
