use axum::{Router, Extension};
use dotenvy::dotenv;
use std::net::SocketAddr;

mod config;

#[tokio::main]
async fn main() {
    
    // Load environment variables from .env file
    dotenv().ok();

    // koneksi ke database
    let db = config::database::connect().await;


    // Buat router dasar
    let app = Router::new()
        .layer(Extension(()));

    // Ambil port dari environment variable, default 3000
    let port = std::env::var("APP_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(3001);

    // Alamat server
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    
    // Tampilkan alamat server di console
    println!("Server running on http://{}", addr);
    
    // Jalankan server
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app
    ).await.unwrap();
}
