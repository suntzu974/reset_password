use axum::{Router};
use std::{net::SocketAddr};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Sert les fichiers de Yew, et renvoie index.html si non trouvé (SPA routing)
   let static_files = ServeDir::new("../frontend/dist");

    // Router sans `nest_service`, mais avec fallback
    let app = Router::new()
    .route("/index", axum::routing::get(|| async { "Hello from index!" }))
    .fallback_service(
            static_files
                .clone()
                .not_found_service(ServeFile::new("../frontend/dist/index.html")),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("→ http://localhost:3000");

  axum::serve(listener, app).await?;
    Ok(())
}
// Note: Ensure you have the necessary dependencies in your Cargo.toml
